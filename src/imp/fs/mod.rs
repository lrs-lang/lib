use imp::syscall::{sync};

pub mod info;

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    sync()
}
