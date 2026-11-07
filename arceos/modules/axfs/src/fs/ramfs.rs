// Mount the teaching RAMFS so changes to its directory operations are used by /tmp.
use alloc::sync::Arc;
use axfs_vfs::{VfsNodeRef, VfsOps, VfsResult};
use axsync::Mutex;

#[path = "../../../../axfs_ramfs/src/dir.rs"]
mod dir;
pub use dir::DirNode;
#[path = "../../../../axfs_ramfs/src/file.rs"]
mod file;

pub struct RamFileSystem(Arc<dir::DirNode>);

impl RamFileSystem {
    pub fn new() -> Self {
        Self(dir::DirNode::new(None))
    }
}

impl VfsOps for RamFileSystem {
    fn mount(&self, _path: &str, mount_point: VfsNodeRef) -> VfsResult {
        self.0.set_parent(mount_point.parent().as_ref());
        Ok(())
    }

    fn root_dir(&self) -> VfsNodeRef {
        self.0.clone()
    }
}
