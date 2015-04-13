// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_file"]
#![crate_type = "lib"]
#![feature(negate_unsigned)]
#![allow(trivial_numeric_casts)]

#[macro_use]
extern crate linux_core as core;
extern crate linux_dev as dev;
extern crate linux_fs as fs;
extern crate linux_time_base as time_base;

use std::{mem};
use std::io::{Write};

use core::result::{Result};
use core::errno::{self, Errno};
use core::cty::{self, c_int, loff_t, c_uint, AT_FDCWD, AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW,
                UTIME_NOW, UTIME_OMIT, timespec, RENAME_EXCHANGE, RENAME_NOREPLACE,
                AT_REMOVEDIR, PATH_MAX, size_t, FALLOC_FL_KEEP_SIZE,
                FALLOC_FL_PUNCH_HOLE, FALLOC_FL_COLLAPSE_RANGE, FALLOC_FL_ZERO_RANGE,
                ssize_t, LOCK_SH, LOCK_EX, LOCK_NB, LOCK_UN};
use core::ext::{AsLinuxPath, UIntRange, BoundedUIntRange};
use core::syscall::{openat, read, write, close, pread, lseek, pwrite, readv, writev,
                    preadv, pwritev, ftruncate, fsync, fdatasync, syncfs, fadvise,
                    fstatfs, fcntl_dupfd_cloexec, fcntl_getfl, fcntl_setfl, fcntl_getfd,
                    fcntl_setfd, fstatat, faccessat, truncate, linkat, utimensat,
                    renameat, mkdirat, unlinkat, symlinkat, readlinkat, fchownat,
                    fchmodat, fchmod, mknodat, readahead, fallocate, setxattr, lsetxattr,
                    fsetxattr, getxattr, lgetxattr, fgetxattr, removexattr, lremovexattr,
                    fremovexattr, listxattr, llistxattr, flistxattr, flock};
use core::string::{AsLinuxStrMut, LinuxStr, LinuxString, AsLinuxStr};
use core::util::{retry, empty_cstr, memchr};
use core::alias::{UserId, GroupId};
use core::c_str::{CStr};
use core::fd_container::{FDContainer, FD};

use time_base::{Time, time_to_timespec};

use fs::info::{FileSystemInfo, from_statfs};

use dev::{Device, DeviceType};

use flags::{Flags, Mode, AccessMode, access_mode_to_int, flags_from_int, flags_to_int,
            mode_to_int};
use info::{Info, info_from_stat, Type, file_type_to_mode};

pub mod flags;
pub mod info;

/// Returns information about the file specified by `path`.
///
/// If `path` is a symlink, then this is equivalent to returning information about the
/// destination of the symlink. Relative paths will be interpreted relative to the current
/// working directory.
pub fn _info<P: AsLinuxPath>(path: P) -> Result<Info> {
    File::current_dir().rel_info(path)
}

/// Returns information about the file specified by `path`.
///
/// This returns information about the file at `path`, even if `path` is a symlink.
/// Relative paths will be interpreted relative to the current working directory.
pub fn info_no_follow<P: AsLinuxPath>(path: P) -> Result<Info> {
    File::current_dir().rel_info_no_follow(path)
}

/// Returns whether the specified path points to an existing file.
///
/// If `path` is relative then the path will be interpreted relative to the current
/// working directory.
pub fn exists<P: AsLinuxPath>(path: P) -> Result<bool> {
    File::current_dir().rel_exists(path)
}

/// Checks whether the file at `path` can be accessed with the specified mode.
///
/// Relative paths are interpreted relative to the current working directory.
pub fn can_access<P: AsLinuxPath>(path: P, mode: AccessMode) -> Result<bool> {
    File::current_dir().rel_can_access(path, mode)
}

/// Sets the length of the file at `path`.
pub fn set_len<P: AsLinuxPath>(path: P, len: u64) -> Result {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf));
    try!(retry(|| truncate(&path, len as loff_t)));
    Ok(())
}

/// Creates a hard link to `old` at `new`.
///
/// If `old` is a symlink then it is not dereferenced. Relative paths are interpreted
/// relative to the current working directory.
pub fn link<P: AsLinuxPath, Q: AsLinuxPath>(old: P, new: Q) -> Result {
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let old = try!(old.to_cstr(&mut buf1));
    let new = try!(new.to_cstr(&mut buf2));
    rv!(linkat(AT_FDCWD, &old, AT_FDCWD, &new, 0))
}

/// Changes the access and modification times of the file specified by `path`.
///
/// Relative paths are interpreted relative to the current working directory. If `path` is
/// a symlink, then this changes the times of the destination.
pub fn set_times<P: AsLinuxPath>(path: P, access: TimeChange,
                                 modification: TimeChange) -> Result {
    File::current_dir().rel_set_times(path, access, modification)
}

/// Changes the access and modification times of the file specified by `path`.
///
/// Relative paths are interpreted relative to the current working directory. If `path` is
/// a symlink, then this changes the times of the symlink.
pub fn set_times_no_follow<P: AsLinuxPath>(path: P, access: TimeChange,
                                           modification: TimeChange) -> Result {
    File::current_dir().rel_set_times_no_follow(path, access, modification)
}

/// Atomically exchanges the two files `one` and `two`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn exchange<P: AsLinuxPath, Q: AsLinuxPath>(one: P, two: Q) -> Result {
    File::current_dir().rel_exchange(one, two)
}

/// Renames `one` to `two`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `replace` is `false`, then the operation fails if `two` already exists.
pub fn rename<P: AsLinuxPath, Q: AsLinuxPath>(one: P, two: Q,
                                              replace: bool) -> Result {
    File::current_dir().rel_rename(one, two, replace)
}

/// Creates the directory `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn create_dir<P: AsLinuxPath>(path: P, mode: Mode) -> Result {
    File::current_dir().rel_create_dir(path, mode)
}

/// Removes the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `path` refers to a directory, then the directory has to be empty.
pub fn remove<P: AsLinuxPath>(path: P) -> Result {
    File::current_dir().rel_remove(path)
}

/// Creates a symlink from `link` to `target`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn symlink<P: AsLinuxPath, Q: AsLinuxPath>(target: P, link: Q) -> Result {
    File::current_dir().rel_symlink(target, link)
}

/// Reads the target of the symbolic link `link` into `buf`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn read_link_buf<P: AsLinuxPath>(link: P, buf: &mut [u8]) -> Result<&mut LinuxStr> {
    File::current_dir().rel_read_link_buf(link, buf)
}

/// Reads the target of the symbolic link `link`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn read_link<P: AsLinuxPath>(link: P) -> Result<LinuxString> {
    File::current_dir().rel_read_link(link)
}

/// Changes the owner of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn change_owner<P: AsLinuxPath>(path: P, user: UserId, group: GroupId) -> Result {
    File::current_dir().rel_change_owner(path, user, group)
}

/// Changes the owner of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `path` refers to a symlink, then this changes the owner of the symlink itself.
pub fn change_owner_no_follow<P: AsLinuxPath>(path: P, user: UserId,
                                              group: GroupId) -> Result {
    File::current_dir().rel_change_owner_no_follow(path, user, group)
}

/// Change the mode of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn change_mode<P: AsLinuxPath>(path: P, mode: Mode) -> Result {
    File::current_dir().rel_change_mode(path, mode)
}

/// Creates a file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
///
/// The type must be one of the following:
///
/// - `File`
/// - `FIFO`
/// - `Socket`
pub fn create_file<P: AsLinuxPath>(path: P, ty: Type, mode: Mode) -> Result {
    File::current_dir().rel_create_file(path, ty, mode)
}

/// Creates a device special file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn create_device<P: AsLinuxPath>(path: P, dev: Device, mode: Mode) -> Result {
    File::current_dir().rel_create_device(path, dev, mode)
}

/// Sets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn set_attr<P: AsLinuxPath, S: AsLinuxStr, V: AsRef<[u8]>>(path: P, name: S,
                                                               val: V) -> Result {
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(setxattr(&path, &name, val.as_ref(), 0))
}

/// Sets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn set_attr_no_follow<P: AsLinuxPath, S: AsLinuxStr, V: AsRef<[u8]>>(
    path: P,
    name: S,
    val: V
    ) -> Result
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(lsetxattr(&path, &name, val.as_ref(), 0))
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn get_attr_buf<P: AsLinuxPath, S: AsLinuxStr, V: AsMut<[u8]>>(
    path: P,
    name: S,
    mut val: V
    ) -> Result<usize>
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(getxattr(&path, &name, val.as_mut()), -> usize)
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn get_attr_no_follow_buf<P: AsLinuxPath, S: AsLinuxStr, V: AsMut<[u8]>>(
        path: P,
        name: S,
        mut val: V
        ) -> Result<usize>
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(lgetxattr(&path, &name, val.as_mut()), -> usize)
}

fn get_attr_common<F: FnMut(&mut [u8]) -> ssize_t>(mut f: F) -> Result<Vec<u8>> {
    let mut vec = vec!();
    loop {
        let size = try!(rv!(f(&mut []), -> usize));
        unsafe {
            vec.set_len(0);
            vec.reserve(size);
            vec.set_len(size);
            match rv!(f(&mut vec[..]), -> usize) {
                Ok(n) => {
                    vec.set_len(n);
                    return Ok(vec);
                },
                Err(errno::RangeError) => { },
                Err(e) => return Err(e),
            }
        }
    }
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn get_attr<P: AsLinuxPath, S: AsLinuxStr>(
    path: P,
    name: S,
    ) -> Result<Vec<u8>>
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    get_attr_common(|buf| getxattr(&path, &name, buf))
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn get_attr_no_follow<P: AsLinuxPath, S: AsLinuxStr>(
    path: P,
    name: S,
    ) -> Result<Vec<u8>>
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    get_attr_common(|buf| lgetxattr(&path, &name, buf))
}

/// Removes an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn remove_attr<P: AsLinuxPath, S: AsLinuxStr>(path: P, name: S) -> Result {
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(removexattr(&path, &name))
}

/// Removes an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn remove_attr_no_follow<P: AsLinuxPath, S: AsLinuxStr>(path: P,
                                                            name: S) -> Result {
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; 128] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf1));
    let name = try!(name.to_cstr(&mut buf2));
    rv!(lremovexattr(&path, &name))
}

/// Returns the buffer size required in a `list_attr_buf` call.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr_size<P: AsLinuxPath>(path: P) -> Result<usize> {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf));
    rv!(listxattr(&path, &mut []), -> usize)
}

/// Returns the buffer size required in a `list_attr_buf_no_follow` call.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_size_no_follow<P: AsLinuxPath>(path: P) -> Result<usize> {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf));
    rv!(llistxattr(&path, &mut []), -> usize)
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr_buf<'a, P: AsLinuxPath>(path: P,
                                         buf: &'a mut [u8]) -> Result<ListAttrIter<'a>> {
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut pbuf));
    let len = try!(rv!(listxattr(&path, buf), -> usize));
    Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_buf_no_follow<'a, P: AsLinuxPath>(
    path: P,
    buf: &'a mut [u8]
    ) -> Result<ListAttrIter<'a>>
{
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut pbuf));
    let len = try!(rv!(llistxattr(&path, buf), -> usize));
    Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
}

fn list_attr_common<F: FnMut(&mut [u8]) -> ssize_t>(mut f: F) -> Result<ListAttrIterator> {
    let mut vec = vec!();
    loop {
        let size = try!(rv!(f(&mut []), -> usize));
        unsafe {
            vec.set_len(0);
            vec.reserve(size);
            vec.set_len(size);
            match rv!(f(&mut vec[..]), -> usize) {
                Ok(n) => {
                    vec.set_len(n);
                    return Ok(ListAttrIterator { buf: vec, pos: 0 });
                },
                Err(errno::RangeError) => { },
                Err(e) => return Err(e),
            }
        }
    }
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr<P: AsLinuxPath>(path: P) -> Result<ListAttrIterator> {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf));
    list_attr_common(|buf| listxattr(&path, buf))
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_no_follow<P: AsLinuxPath>(path: P) -> Result<ListAttrIterator> {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let path = try!(path.to_cstr(&mut buf));
    list_attr_common(|buf| llistxattr(&path, buf))
}

/// An opened file in a file system.
#[derive(Debug, Eq, PartialEq)]
pub struct File {
    fd: c_int,
    /// File has ownership of the file descriptor.
    owned: bool,
}

impl File {
    /// Creates a file on which every operation fails.
    pub fn invalid() -> File {
        File { fd: -1, owned: false }
    }

    /// Creates a file that points to the current directory.
    pub fn current_dir() -> File {
        File { fd: AT_FDCWD, owned: false }
    }

    /// Opens the file at path `path` in read mode.
    ///
    /// This is equivalent to `File::open` with the default flags.
    pub fn open_read<P: AsLinuxPath>(path: P) -> Result<File> {
        File::current_dir().rel_open_read(path)
    }

    /// Open the file at path `path` with the specified flags.
    pub fn open<P: AsLinuxPath>(path: P, flags: Flags) -> Result<File> {
        File::current_dir().rel_open(path, flags)
    }

    /// Reads bytes from the current read position into the buffer.
    ///
    /// ### Return value
    ///
    /// Returns the number of bytes read or an error. Zero indicates end of file.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        retry(|| read(self.fd, buf)).map(|r| r as usize)
    }

    /// Writes bytes to the current to position from the buffer.
    ///
    /// ### Return value
    ///
    /// Returns the number of bytes written or an error.
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.fd, buf)).map(|r| r as usize)
    }

    /// Closes the file descriptor.
    pub fn close(&mut self) -> Result {
        if self.owned {
            let ret = close(self.fd);
            self.fd = -1;
            rv!(ret)
        } else {
            Ok(())
        }
    }

    /// Returns information about the file.
    pub fn info(&self) -> Result<Info> {
        let mut stat = unsafe { mem::zeroed() };
        try!(rv!(fstatat(self.fd, empty_cstr(), &mut stat, AT_EMPTY_PATH)));
        Ok(info_from_stat(stat))
    }

    /// Performs requested seek operation.
    ///
    /// ### Return value
    ///
    /// Returns the new position in the file or an error.
    pub fn seek(&self, pos: Seek) -> Result<i64> {
        let ret = lseek(self.fd, pos.offset(), pos.whence());
        rv!(ret, -> i64)
    }

    /// Creates a new file referring to the same file description.
    ///
    /// The `close on exec` flag will be set on the new file.
    ///
    /// ### Return value
    ///
    /// Returns the new file or an error.
    pub fn duplicate(&self) -> Result<File> {
        let new_fd = fcntl_dupfd_cloexec(self.fd, 0);
        if new_fd < 0 {
            Err(Errno(-new_fd as c_int))
        } else {
            Ok(File { fd: new_fd, owned: true })
        }
    }

    /// Retrieves the file description flags.
    ///
    /// The returned flags will contain the access mode flags and the file status flags.
    pub fn get_status_flags(&self) -> Result<Flags> {
        let ret = fcntl_getfl(self.fd);
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(flags_from_int(ret))
        }
    }

    /// Sets the file description flags.
    ///
    /// Only the file status flags can be modified.
    pub fn set_status_flags(&self, flags: Flags) -> Result {
        let ret = fcntl_setfl(self.fd, flags_to_int(flags));
        rv!(ret)
    }

    /// Returns whether the file has the `close on exec` flag set.
    pub fn is_close_on_exec(&self) -> Result<bool> {
        let ret = fcntl_getfd(self.fd);
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(ret & cty::O_CLOEXEC != 0)
        }
    }

    /// Modifies the `close on exec` flag of the file.
    pub fn set_close_on_exec(&self, val: bool) -> Result {
        let mut ret = fcntl_getfd(self.fd);
        if ret >= 0 {
            ret = (ret & !cty::O_CLOEXEC) | (cty::O_CLOEXEC * val as c_int);
            ret = fcntl_setfd(self.fd, ret);
        }
        rv!(ret)
    }

    /// Reads bytes from the offset into the buffer.
    ///
    /// The return value and errors are the same as for `read` and `seek`.
    pub fn read_at(&self, buf: &mut [u8], off: i64) -> Result<usize> {
        retry(|| pread(self.fd, buf, off as loff_t)).map(|r| r as usize)
    }

    /// Writes bytes to the offset from the buffer.
    ///
    /// The return value and errors are the same as for `write` and `seek`.
    pub fn write_at(&self, buf: &[u8], off: i64) -> Result<usize> {
        retry(|| pwrite(self.fd, buf, off as loff_t)).map(|r| r as usize)
    }

    /// Reads bytes from the current read position into the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes read.
    pub fn scatter_read(&self, bufs: &mut [&mut [u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| readv(self.fd, bufs)).map(|r| r as usize)
    }

    /// Writes bytes to the current write position from the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes written.
    pub fn gather_write(&self, bufs: &[&[u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| writev(self.fd, bufs)).map(|r| r as usize)
    }

    /// Reads bytes from the offset into the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes read.
    pub fn scatter_read_at(&self, bufs: &mut [&mut [u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| preadv(self.fd, bufs, off as loff_t)).map(|r| r as usize)
    }

    /// Writes bytes to the offset from the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes written.
    pub fn gather_write_at(&self, bufs: &[&[u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| pwritev(self.fd, bufs, off as loff_t)).map(|r| r as usize)
    }

    /// Changes the length of the file to the specified length.
    ///
    /// If the requested length is larger than the current length, a hole is created.
    pub fn set_len(&self, len: i64) -> Result {
        retry(|| ftruncate(self.fd, len as loff_t)).map(|_| ())
    }

    /// Flushes all data and metadata to the disk.
    pub fn sync(&self) -> Result {
        rv!(fsync(self.fd))
    }

    /// Flushes enough data to the disk that the content of the file can be read again.
    pub fn data_sync(&self) -> Result {
        rv!(fdatasync(self.fd))
    }

    /// Writes all data and metadata of the filesystem containing this file to the disk.
    pub fn sync_filesystem(&self) -> Result {
        rv!(syncfs(self.fd))
    }

    /// Advise the kernel that the specified range will have a certain usage pattern.
    pub fn advise<R: UIntRange<u64>>(&self, range: R, advice: Advice) -> Result {
        let range = range.to_range();
        let len = match range.end {
            -1 => 0,
            _ => range.end - range.start,
        };
        let ret = fadvise(self.fd, range.start as loff_t, len as loff_t, advice.to_c_int());
        rv!(ret)
    }

    /// Returns information about the file system of this file.
    pub fn fs_info(&self) -> Result<FileSystemInfo> {
        let mut buf = unsafe { mem::zeroed() };
        retry(|| fstatfs(self.fd, &mut buf)).map(|_| from_statfs(buf))
    }

    /// Creates a hard link to this file.
    ///
    /// Relative paths are interpreted relative to the current working directory.
    pub fn link<P: AsLinuxPath>(&self, path: P) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(linkat(self.fd, empty_cstr(), AT_FDCWD, &path, AT_EMPTY_PATH))
    }

    /// Creates a hard link to this file relative to a directory.
    ///
    /// Relative paths are interpreted relative to the directory `dir`.
    pub fn link_rel_to<P: AsLinuxPath>(&self, dir: &File, path: P) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(linkat(self.fd, empty_cstr(), dir.fd, &path, AT_EMPTY_PATH))
    }

    /// Changes the access and modification times of this file.
    pub fn set_times(&self, access: TimeChange, modification: TimeChange) -> Result {
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, None, &times, 0))
    }

    /// Returns the path of the file that was used to open this file.
    pub fn filename_buf<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut LinuxStr> {
        // enough space for "/proc/self/fd/-{u64::MAX}\0"
        let mut proc_buf = [0; 36];
        let _ = write!(&mut proc_buf[..], "/proc/self/fd/{}", self.fd);
        // FIXME: not actually correct
        let cstr = unsafe { CStr::from_nt_slice_mut(&mut proc_buf) };
        let len = try!(rv!(readlinkat(self.fd, cstr, buf), -> usize));
        Ok(buf[..len].as_linux_str_mut())
    }

    /// Returns the path of the file that was used to open this file.
    pub fn filename(&self) -> Result<LinuxString> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        self.filename_buf(&mut buf).map(|f| f.to_linux_string())
    }

    /// Changes the owner of this file.
    pub fn change_owner(&self, user: UserId, group: GroupId) -> Result {
        rv!(fchownat(self.fd, empty_cstr(), user, group, AT_EMPTY_PATH))
    }

    /// Changes the mode of this file.
    pub fn change_mode(&self, mode: Mode) -> Result {
        rv!(fchmod(self.fd, mode_to_int(mode)))
    }

    /// Initiates readahead of the specified range.
    pub fn readahead<R: BoundedUIntRange<u64>>(&self, range: R) -> Result {
        let range = range.to_range();
        rv!(readahead(self.fd, range.start as loff_t, (range.end - range.start) as size_t))
    }

    /// Reserves the specified range in the file system.
    ///
    /// Further writes in the specified range are guaranteed not to fail because of a lack
    /// of storage capacity.
    pub fn reserve<R: BoundedUIntRange<u64>>(&self, range: R) -> Result {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_KEEP_SIZE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }

    /// Creates a hole in the specified range.
    pub fn create_hole<R: BoundedUIntRange<u64>>(&self, range: R) -> Result {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE,
                      range.start as loff_t, (range.end - range.start) as loff_t))
    }

    /// Removes the specified range from the file and closes the gap.
    ///
    /// The range must probably begin and end at a multiple of the block size but this
    /// depends on the filesystem. This function cannot be used if the range reaches the
    /// end of the file. Use `set_len` for this purpose.
    pub fn collapse<R: BoundedUIntRange<u64>>(&self, range: R) -> Result {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_COLLAPSE_RANGE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }
    
    /// Zeroes the specified range in the file.
    /// 
    /// This can be more efficient than manually writing zeroes.
    pub fn zero<R: BoundedUIntRange<u64>>(&self, range: R) -> Result {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_ZERO_RANGE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }

    /// Sets an attribute of this file.
    pub fn set_attr<S: AsLinuxStr, V: AsRef<[u8]>>(&self, name: S, val: V) -> Result {
        let mut buf: [u8; 128] = unsafe { mem::uninitialized() };
        let name = try!(name.to_cstr(&mut buf));
        rv!(fsetxattr(self.fd, &name, val.as_ref(), 0))
    }

    /// Gets an attribute of this file.
    pub fn get_attr_buf<S: AsLinuxStr, V: AsMut<[u8]>>(&self, name: S,
                                                       mut val: V) -> Result<usize> {
        let mut buf: [u8; 128] = unsafe { mem::uninitialized() };
        let name = try!(name.to_cstr(&mut buf));
        rv!(fgetxattr(self.fd, &name, val.as_mut()), -> usize)
    }

    /// Gets an attribute of this file.
    pub fn get_attr<S: AsLinuxStr>(&self, name: S) -> Result<Vec<u8>> {
        let mut buf: [u8; 128] = unsafe { mem::uninitialized() };
        let name = try!(name.to_cstr(&mut buf));
        get_attr_common(|buf| fgetxattr(self.fd, &name, buf))
    }

    /// Removes an attribute of this file.
    pub fn remove_attr<S: AsLinuxStr>(&self, name: S) -> Result {
        let mut buf: [u8; 128] = unsafe { mem::uninitialized() };
        let name = try!(name.to_cstr(&mut buf));
        rv!(fremovexattr(self.fd, &name))
    }

    /// Returns the buffer size required in a `list_attr_buf` call.
    pub fn list_attr_size(&self) -> Result<usize> {
        rv!(flistxattr(self.fd, &mut []), -> usize)
    }

    /// Returns an iterator over the attributes in this file.
    pub fn list_attr_buf<'a>(&self, buf: &'a mut [u8]) -> Result<ListAttrIter<'a>> {
        let len = try!(rv!(flistxattr(self.fd, buf), -> usize));
        Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
    }

    /// Returns an iterator over the attributes in this file.
    pub fn list_attr(&self) -> Result<ListAttrIterator> {
        list_attr_common(|buf| flistxattr(self.fd, buf))
    }

    /// Tries to lock this file exclusively without blocking.
    pub fn try_lock_exclusive(&self) -> Result {
        rv!(flock(self.fd, LOCK_EX | LOCK_NB))
    }

    /// Tries to lock this file exclusively.
    pub fn lock_exclusive(&self) -> Result {
        retry(|| flock(self.fd, LOCK_EX)).map(|_| ())
    }

    /// Tries to lock this file shared without blocking.
    pub fn try_lock_shared(&self) -> Result {
        rv!(flock(self.fd, LOCK_SH | LOCK_NB))
    }

    /// Tries to lock this file shared.
    pub fn lock_shared(&self) -> Result {
        retry(|| flock(self.fd, LOCK_SH)).map(|_| ())
    }

    /// Unlocks this file.
    pub fn unlock(&self) -> Result {
        rv!(flock(self.fd, LOCK_UN))
    }
}

impl File {
    /// Opens the file at path `path` in read mode.
    ///
    /// If `path` is relative, the `self` must be a directory and the `path` will be
    /// interpreted relative to `self`.
    ///
    /// This is equivalent to `file.open` with the default flags.
    pub fn rel_open_read<P: AsLinuxPath>(&self, path: P) -> Result<File> {
        self.rel_open(path, Flags::new())
    }

    /// Open the file at path `path` with the specified flags.
    ///
    /// If `path` is relative, the `self` must be a directory and the `path` will be
    /// interpreted relative to `self`.
    pub fn rel_open<P: AsLinuxPath>(&self, path: P, flags: Flags) -> Result<File> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let mode = flags.mode().map(|m| mode_to_int(m)).unwrap_or(0);
        let fd = match retry(|| openat(self.fd, &path,
                                       flags_to_int(flags) | cty::O_LARGEFILE, mode)) {
            Ok(fd) => fd,
            // Due to a bug in the kernel, open returns WrongDeviceType instead of
            // NoSuchDevice.
            Err(errno::WrongDeviceType) => return Err(errno::NoSuchDevice),
            Err(e) => return Err(e),
        };
        Ok(File {
            fd: fd,
            owned: true,
        })
    }

    /// Returns information about the file specified by `path`.
    ///
    /// If `path` is a symlink, then this is equivalent to returning information about the
    /// destination of the symlink. If `path` is relative, then `self` must be a directory
    /// and the path will be interpreted relative to `self`.
    pub fn rel_info<P: AsLinuxPath>(&self, path: P) -> Result<Info> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let mut stat = unsafe  { mem::zeroed() };
        try!(rv!(fstatat(self.fd, &path, &mut stat, 0)));
        Ok(info_from_stat(stat))
    }

    /// Returns information about the file specified by `path`.
    ///
    /// This returns information about the file at `path`, even if `path` is a symlink.
    /// If `path` is relative, then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_info_no_follow<P: AsLinuxPath>(&self, path: P) -> Result<Info> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let mut stat = unsafe  { mem::zeroed() };
        try!(rv!(fstatat(self.fd, &path, &mut stat, AT_SYMLINK_NOFOLLOW)));
        Ok(info_from_stat(stat))
    }

    /// Returns whether the specified path points to an existing file.
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_exists<P: AsLinuxPath>(&self, path: P) -> Result<bool> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let res = faccessat(self.fd, &path, 0);
        if res >= 0 {
            Ok(true)
        } else {
            let err = Errno(-res);
            match err {
                errno::DoesNotExist => Ok(false),
                _ => Err(err),
            }
        }
    }

    /// Checks whether the file at `path` can be accessed with the specified mode.
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_can_access<P: AsLinuxPath>(&self, path: P,
                                          mode: AccessMode) -> Result<bool> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let res = faccessat(self.fd, &path, access_mode_to_int(mode));
        if res >= 0 {
            Ok(true)
        } else {
            let err = Errno(-res);
            match err {
                errno::AccessDenied => Ok(false),
                _ => Err(err),
            }
        }
    }

    /// Changes the access and modification times of the file specified by `path`.
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the destination.
    pub fn rel_set_times<P: AsLinuxPath>(&self, path: P, access: TimeChange,
                                         modification: TimeChange) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, 0))
    }

    /// Changes the access and modification times of the file specified by `path`.
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the symlink.
    pub fn rel_set_times_no_follow<P: AsLinuxPath>(&self, path: P, access: TimeChange,
                                                   modification: TimeChange) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, AT_SYMLINK_NOFOLLOW))
    }

    /// Atomically exchanges the two files `one` and `two`.
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`.
    pub fn rel_exchange<P: AsLinuxPath, Q: AsLinuxPath>(&self, one: P,
                                                        two: Q) -> Result {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let one = try!(one.to_cstr(&mut buf1));
        let two = try!(two.to_cstr(&mut buf2));
        rv!(renameat(self.fd, &one, self.fd, &two, RENAME_EXCHANGE))
    }

    /// Renames `one` to `two`.
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`. If `replace` is `false`, then the
    /// operation fails if `two` already exists.
    pub fn rel_rename<P: AsLinuxPath, Q: AsLinuxPath>(&self, one: P, two: Q,
                                                      replace: bool) -> Result {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let one = try!(one.to_cstr(&mut buf1));
        let two = try!(two.to_cstr(&mut buf2));
        let flag = if replace { 0 } else { RENAME_NOREPLACE };
        rv!(renameat(self.fd, &one, self.fd, &two, flag))
    }

    /// Creates the directory `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`.
    pub fn rel_create_dir<P: AsLinuxPath>(&self, path: P, mode: Mode) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(mkdirat(self.fd, &path, mode_to_int(mode)))
    }

    /// Removes the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`. If `path` refers to a directory, then the
    /// directory has to be empty.
    pub fn rel_remove<P: AsLinuxPath>(&self, path: P) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        let mut ret = unlinkat(self.fd, &path, 0);
        if Errno(-ret) == errno::IsADirectory {
            ret = unlinkat(self.fd, &path, AT_REMOVEDIR);
        }
        rv!(ret)
    }

    /// Creates a symlink from `link` to `target`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_symlink<P: AsLinuxPath, Q: AsLinuxPath>(&self, target: P,
                                                       link: Q) -> Result {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let target = try!(target.to_cstr(&mut buf1));
        let link = try!(link.to_cstr(&mut buf2));
        rv!(symlinkat(&target, self.fd, &link))
    }

    /// Reads the target of the symbolic link `link` into `buf`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_read_link_buf<'a, P: AsLinuxPath>(
            &self,
            link: P,
            buf: &'a mut [u8]
            ) -> Result<&'a mut LinuxStr>
    {
        let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let link = try!(link.to_cstr(&mut pbuf));
        let len = try!(rv!(readlinkat(self.fd, &link, buf), -> usize));
        Ok(buf[..len].as_linux_str_mut())
    }

    /// Reads the target of the symbolic link `link`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_read_link<P: AsLinuxPath>(&self, link: P) -> Result<LinuxString> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        self.rel_read_link_buf(link, &mut buf).map(|f| f.to_linux_string())
    }

    /// Changes the owner of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_owner<P: AsLinuxPath>(&self, path: P, user: UserId,
                                            group: GroupId) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, 0))
    }

    /// Changes the owner of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`. If `path` refers to a symlink, then this changes
    /// the owner of the symlink itself.
    pub fn rel_change_owner_no_follow<P: AsLinuxPath>(&self, path: P, user: UserId,
                                                      group: GroupId) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, AT_SYMLINK_NOFOLLOW))
    }

    /// Change the mode of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_mode<P: AsLinuxPath>(&self, path: P, mode: Mode) -> Result {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(fchmodat(self.fd, &path, mode_to_int(mode)))
    }

    /// Creates a file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    ///
    /// The type must be one of the following:
    ///
    /// - `File`
    /// - `FIFO`
    /// - `Socket`
    pub fn rel_create_file<P: AsLinuxPath>(&self, path: P, ty: Type,
                                         mode: Mode) -> Result {
        match ty {
            Type::File | Type::FIFO | Type::Socket => { },
            _ => return Err(errno::InvalidArgument),
        }
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), 0))
    }

    /// Creates a device special file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_create_device<P: AsLinuxPath>(&self, path: P, dev: Device,
                                           mode: Mode) -> Result {
        let ty = match dev.ty() {
            DeviceType::Character => Type::CharDevice,
            DeviceType::Block     => Type::BlockDevice,
        };
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
        let path = try!(path.to_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), dev.id()))
    }
}

impl ::std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        Ok(try!(File::read(self, buf)))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl FDContainer for File {
    fn unwrap(self) -> FD {
        let fd = self.fd;
        unsafe { mem::forget(self); }
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> FD {
        self.fd
    }

    fn from_owned(fd: FD) -> File {
        File { fd: fd, owned: true }
    }

    fn from_borrowed(fd: FD) -> File {
        File { fd: fd, owned: false }
    }
}

/// Enum used to specify the way time information of a file is modified.
pub enum TimeChange {
    /// Donesn't modify the time.
    Omit,
    /// Sets the time to the current time.
    Now,
    /// Sets the time to the specified time.
    Set(Time),
}

fn time_change_to_timespec(t: TimeChange) -> timespec {
    match t {
        TimeChange::Omit => timespec { tv_sec: 0, tv_nsec: UTIME_OMIT },
        TimeChange::Now  => timespec { tv_sec: 0, tv_nsec: UTIME_NOW  },
        TimeChange::Set(v) => time_to_timespec(v),
    }
}

/// A seek operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Seek {
    /// Seek from the start of the file.
    Start(i64),
    /// Seek from the current position in the file.
    Cur(i64),
    /// Seek from the end of the file.
    End(i64),
    /// Seek to the first non-hole byte at or after the specified offset.
    Data(i64),
    /// Seek to the first hole at or after the specified offset.
    Hole(i64),
}

impl Seek {
    fn whence(self) -> c_uint {
        match self {
            Seek::Start(..) => cty::SEEK_SET,
            Seek::Cur(..)   => cty::SEEK_CUR,
            Seek::End(..)   => cty::SEEK_END,
            Seek::Data(..)  => cty::SEEK_DATA,
            Seek::Hole(..)  => cty::SEEK_HOLE,
        }
    }

    fn offset(self) -> loff_t {
        match self {
            Seek::Start(v) => v as loff_t,
            Seek::Cur(v)   => v as loff_t,
            Seek::End(v)   => v as loff_t,
            Seek::Data(v)  => v as loff_t,
            Seek::Hole(v)  => v as loff_t,
        }
    }
}

/// Advice used to optimize file access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Advice {
    /// Default.
    Normal,
    /// Optimize for random access.
    Random,
    /// Optimize for sequential access.
    Sequential,
    /// The range will be accessed soon.
    Need,
    /// The range will not be accessed.
    DontNeed,
    /// The range will be accessed only once.
    NoReuse,
}

impl Advice {
    fn to_c_int(self) -> c_int {
        match self {
            Advice::Normal     => cty::POSIX_FADV_NORMAL,
            Advice::Random     => cty::POSIX_FADV_RANDOM,
            Advice::Sequential => cty::POSIX_FADV_SEQUENTIAL,
            Advice::Need       => cty::POSIX_FADV_WILLNEED,
            Advice::DontNeed   => cty::POSIX_FADV_DONTNEED,
            Advice::NoReuse    => cty::POSIX_FADV_NOREUSE,
        }
    }
}

pub struct ListAttrIter<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ListAttrIter<'a> {
    type Item = &'a LinuxStr;

    fn next(&mut self) -> Option<&'a LinuxStr> {
        if self.pos == self.buf.len() {
            return None;
        }
        let buf = &self.buf[self.pos..];
        let len = memchr(buf, 0).unwrap();
        self.pos += len + 1;
        Some(buf[..len].as_linux_str())
    }
}

pub struct ListAttrIterator {
    buf: Vec<u8>,
    pos: usize,
}

impl Iterator for ListAttrIterator {
    type Item = LinuxString;

    fn next(&mut self) -> Option<LinuxString> {
        if self.pos == self.buf.len() {
            return None;
        }
        let buf = &self.buf[self.pos..];
        let len = memchr(buf, 0).unwrap();
        self.pos += len + 1;
        Some(buf[..len].as_linux_str().to_linux_string())
    }
}
