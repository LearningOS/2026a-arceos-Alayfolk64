#![no_std]

use core::{alloc::Layout, ptr::NonNull};
use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    bytes: usize,
    pages: usize,
    live: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self { start: 0, end: 0, bytes: 0, pages: 0, live: 0 }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start.checked_add(size).expect("invalid memory range");
        self.bytes = start;
        self.pages = self.end;
        self.live = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        // A bump allocator can only extend its existing contiguous range.
        if start != self.end || self.pages != self.end {
            return Err(AllocError::InvalidParam);
        }
        self.end = start.checked_add(size).ok_or(AllocError::InvalidParam)?;
        self.pages = self.end;
        Ok(())
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let start = self.bytes.checked_next_multiple_of(layout.align()).ok_or(AllocError::NoMemory)?;
        let end = start.checked_add(layout.size()).ok_or(AllocError::NoMemory)?;
        if end > self.pages { return Err(AllocError::NoMemory); }
        let ptr = NonNull::new(start as *mut u8).ok_or(AllocError::InvalidParam)?;
        self.bytes = end;
        self.live += 1;
        Ok(ptr)
    }

    fn dealloc(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        self.live -= 1;
        if self.live == 0 { self.bytes = self.start; }
    }

    fn total_bytes(&self) -> usize { self.pages - self.start }
    fn used_bytes(&self) -> usize { self.bytes - self.start }
    fn available_bytes(&self) -> usize { self.pages - self.bytes }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(&mut self, count: usize, align: usize) -> AllocResult<usize> {
        if count == 0 || !align.is_power_of_two() { return Err(AllocError::InvalidParam); }
        let size = count.checked_mul(SIZE).ok_or(AllocError::NoMemory)?;
        let start = self.pages.checked_sub(size).ok_or(AllocError::NoMemory)? & !(align.max(SIZE) - 1);
        if start < self.bytes { return Err(AllocError::NoMemory); }
        self.pages = start;
        Ok(start)
    }

    fn dealloc_pages(&mut self, _start: usize, _count: usize) {
        // Early-boot pages live until the formal allocator takes over.
    }

    fn total_pages(&self) -> usize { self.used_pages() + self.available_pages() }
    fn used_pages(&self) -> usize { (self.end - self.pages) / SIZE }
    fn available_pages(&self) -> usize { self.available_bytes() / SIZE }
}
