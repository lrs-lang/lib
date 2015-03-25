// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::{Debug, Formatter, Error};
use std::{self, mem};

use core::cty::{statfs};
use core::syscall::{statfs};
use core::util::{retry};
use core::ext::{AsLinuxPath};
use core::result::{Result};

use self::mount::{Flags};
use self::types::{FileSystem};

pub mod types;
pub mod mount;

pub fn from_statfs(s: statfs) -> FileSystemInfo {
    FileSystemInfo(s)
}

/// Filesystem information.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FileSystemInfo(statfs);

impl FileSystemInfo {
    /// Returns information about the filesystem located at the path.
    pub fn from_path<P: AsLinuxPath>(path: P) -> Result<FileSystemInfo> {
        let path = path.to_cstring().unwrap();
        let mut buf = unsafe { mem::zeroed() };
        retry(|| statfs(&path, &mut buf)).map(|_| FileSystemInfo(buf))
    }

    /// Returns the type of the filesystem.
    pub fn ty(&self) -> FileSystem {
        FileSystem(self.0.f_type)
    }

    /// Returns the block size of the filesystem.
    pub fn block_size(&self) -> u64 {
        self.0.f_bsize as u64
    }

    /// Returns the number of blocks in the filesystem.
    pub fn blocks(&self) -> u64 {
        self.0.f_blocks as u64
    }

    /// Returns the number of free blocks in the filesystem.
    pub fn free_blocks(&self) -> u64 {
        self.0.f_bfree as u64
    }

    /// Returns the number of free blocks usable by unprivileged users.
    pub fn available_blocks(&self) -> u64 {
        self.0.f_bavail as u64
    }

    /// Returns the number of files in the filesystem.
    pub fn files(&self) -> u64 {
        self.0.f_files as u64
    }

    /// Returns the number of free inodes in the filesystem.
    pub fn free_files(&self) -> u64 {
        self.0.f_ffree as u64
    }

    /// Returns the maximum length of a filename in the filesystem.
    pub fn max_name_len(&self) -> u64 {
        self.0.f_namelen as u64
    }

    /// Returns the fragment size of the filesystem.
    pub fn fragment_size(&self) -> u64 {
        self.0.f_frsize as u64
    }

    /// Returns the flags the filesystem is mounted with.
    pub fn mount_flags(&self) -> Flags {
        Flags(self.0.f_frsize)
    }
}

impl Debug for FileSystemInfo {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), Error> {
        write!(f, "Flags {{ ty: {:?}, block_size: {}, blocks: {}, free_blocks: {}, \
                           available_blocks: {}, files: {}, free_files: {}, \
                           max_name_len: {}, fragment_size: {}, mount_flags: {:?} }}",
                   self.ty(), self.block_size(), self.blocks(), self.free_blocks(),
                   self.available_blocks(), self.files(), self.free_files(),
                   self.max_name_len(), self.fragment_size(), self.mount_flags())
    }
}
