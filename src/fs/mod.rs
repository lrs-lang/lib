use libc::{sync};

pub use self::file::{File};

pub mod file;
pub mod info;

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    unsafe { sync() }
}
