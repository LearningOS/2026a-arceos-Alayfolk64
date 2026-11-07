pub use alloc::collections::*;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

// FNV-1a: mix each byte into a 64-bit hash.
struct KeyHash(u64);

impl Hasher for KeyHash {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(&self) -> u64 { self.0 }
}

pub struct HashMap<K, V> {
    slots: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self { slots: Vec::new(), len: 0 }
    }

    // Probe consecutive slots until we reach the key or the first empty slot.
    fn slot(&self, key: &K) -> usize {
        let mut hash = KeyHash(0xcbf29ce484222325);
        key.hash(&mut hash);
        let mask = self.slots.len() - 1;
        let mut index = hash.finish() as usize & mask;
        while let Some((stored, _)) = &self.slots[index] {
            if stored == key { break; }
            index = (index + 1) & mask;
        }
        index
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len * 2 >= self.slots.len() {
            let old = core::mem::take(&mut self.slots);
            self.slots.resize_with((2 * old.len()).max(8), || None);
            for (k, v) in old.into_iter().flatten() {
                let i = self.slot(&k);
                self.slots[i] = Some((k, v));
            }
        }
        let index = self.slot(&key);
        if let Some((_, old)) = &mut self.slots[index] {
            return Some(core::mem::replace(old, value));
        }
        self.slots[index] = Some((key, value));
        self.len += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.len == 0 { return None; }
        self.slots[self.slot(key)].as_ref().map(|(_, value)| value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.slots.iter().flatten()
    }
}
