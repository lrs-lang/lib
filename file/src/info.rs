// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fmt};

use core::cty::{stat, S_IFMT, S_IFDIR, S_IFCHR, S_IFBLK, S_IFREG, S_IFIFO, S_IFLNK,
                S_IFSOCK, mode_t};
use core::alias::{InodeId, UserId, GroupId};
use clock::{Duration, duration_from_timespec};
use dev::{Device, DeviceType};
use flags::{Mode};

pub fn info_from_stat(s: stat) -> Info { Info(s) }

pub fn file_type_from_mode(i: mode_t) -> Type {
    match i & S_IFMT {
        S_IFIFO  => Type::FIFO,
        S_IFCHR  => Type::CharDevice,
        S_IFDIR  => Type::Directory,
        S_IFBLK  => Type::BlockDevice,
        S_IFREG  => Type::File,
        S_IFLNK  => Type::SymLink,
        S_IFSOCK => Type::Socket,
        _  => Type::Unknown,
    }
}

/// Type of a directory entry.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    /// A block device.
    BlockDevice,
    /// A character device.
    CharDevice,
    /// A directory.
    Directory,
    /// A named pipe.
    FIFO,
    /// A symbolic link.
    SymLink,
    /// A regular file.
    File,
    /// A UNIX domain socket.
    Socket,
    /// Unknown
    Unknown,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info(stat);

impl Info {
    pub fn device(&self) -> Device {
        Device::from_id(self.0.st_dev, DeviceType::Block)
    }

    pub fn inode(&self) -> InodeId {
        self.0.st_ino
    }

    pub fn nr_hard_links(&self) -> u64 {
        self.0.st_nlink as u64
    }

    pub fn mode(&self) -> Mode {
        Mode::from_mode(self.0.st_mode)
    }

    pub fn user(&self) -> UserId {
        self.0.st_uid
    }

    pub fn group(&self) -> GroupId {
        self.0.st_gid
    }

    pub fn special_file(&self) -> Option<Device> {
        match self.file_type() {
            Type::BlockDevice => Some(Device::from_id(self.0.st_rdev, DeviceType::Block)),
            Type::CharDevice => Some(Device::from_id(self.0.st_rdev, DeviceType::Character)),
            _ => None,
        }
    }

    pub fn size(&self) -> u64 {
        self.0.st_size as u64
    }

    pub fn blocks(&self) -> u64 {
        self.0.st_blocks as u64
    }

    pub fn preferred_write_size(&self) -> u64 {
        self.0.st_blksize as u64
    }

    pub fn last_access(&self) -> Duration {
        duration_from_timespec(self.0.st_atim)
    }

    pub fn last_modification(&self) -> Duration {
        duration_from_timespec(self.0.st_mtim)
    }

    pub fn creation(&self) -> Duration {
        duration_from_timespec(self.0.st_ctim)
    }

    pub fn file_type(&self) -> Type {
        file_type_from_mode(self.0.st_mode)
    }
}

impl fmt::Debug for Info {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Info {{ device: {:?}, inode: {}, nr_hard_links: {}, mode: {:?}, \
                     user: {}, group: {}, special_file: {:?}, size: {}, blocks: {}, \
                     preferred_write_size: {}, last_access: {:?}, \
                     last_modification: {:?}, creation: {:?}, file_type: {:?} }}",
                     self.device(), self.inode(), self.nr_hard_links(), self.mode(),
                     self.user(), self.group(), self.special_file(), self.size(),
                     self.blocks(), self.preferred_write_size(), self.last_access(),
                     self.last_modification(), self.creation(), self.file_type())
    }
}