// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_file"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core      as core;
extern crate linux_base      as base;
extern crate linux_io        as io;
extern crate linux_cty       as cty;
extern crate linux_int       as int;
extern crate linux_syscall   as syscall;
extern crate linux_str_one   as str_one;
extern crate linux_str_two   as str_two;
extern crate linux_str_three as str_three;
extern crate linux_arch_fns  as arch_fns;
extern crate linux_rv        as rv;
extern crate linux_fmt       as fmt;
extern crate linux_vec       as vec;
extern crate linux_rmo       as rmo;
extern crate linux_parse     as parse;
extern crate linux_fd        as fd;
extern crate linux_dev       as dev;
extern crate linux_fs        as fs;
extern crate linux_time_base as time_base;

#[prelude_import] use base::prelude::*;
mod linux { pub use vec::linux::*; pub use {cty}; }

use vec::{SVec};
use core::{mem};
use io::{Read};
use base::rmo::{AsRef, AsMut};
use base::error::{self, Errno};
use cty::{
    c_int, loff_t, c_uint, AT_FDCWD, AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW, UTIME_NOW,
    UTIME_OMIT, timespec, RENAME_EXCHANGE, RENAME_NOREPLACE, AT_REMOVEDIR, PATH_MAX,
    size_t, FALLOC_FL_KEEP_SIZE, FALLOC_FL_PUNCH_HOLE, FALLOC_FL_COLLAPSE_RANGE,
    FALLOC_FL_ZERO_RANGE, ssize_t, LOCK_SH, LOCK_EX, LOCK_NB, LOCK_UN
};
use int::{BoundedRange};
use syscall::{
    openat, read, write, close, pread, lseek, pwrite, readv, writev, preadv, pwritev,
    ftruncate, fsync, fdatasync, syncfs, fadvise, fstatfs, fcntl_dupfd_cloexec,
    fcntl_getfl, fcntl_setfl, fcntl_getfd, fcntl_setfd, fstatat, faccessat, truncate,
    linkat, utimensat, renameat, mkdirat, unlinkat, symlinkat, readlinkat, fchownat,
    fchmodat, fchmod, mknodat, readahead, fallocate, setxattr, lsetxattr, fsetxattr,
    getxattr, lgetxattr, fgetxattr, removexattr, lremovexattr, fremovexattr, listxattr,
    llistxattr, flistxattr, flock
};
use str_one::{AsCStr, CStr, ByteStr, AsByteStr, NoNullStr, AsMutNoNullStr};
use str_two::{ByteString, NoNullString};
use str_three::{ToCString};
use arch_fns::{memchr};
use rv::{retry};
use cty::alias::{UserId, GroupId};
use fd::{FDContainer};
use rmo::{ToOwned};

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
pub fn _info<P>(path: P) -> Result<Info>
    where P: ToCString,
{
    File::current_dir().rel_info(path)
}

/// Returns information about the file specified by `path`.
///
/// This returns information about the file at `path`, even if `path` is a symlink.
/// Relative paths will be interpreted relative to the current working directory.
pub fn info_no_follow<P>(path: P) -> Result<Info>
    where P: ToCString,
{
    File::current_dir().rel_info_no_follow(path)
}

/// Returns whether the specified path points to an existing file.
///
/// If `path` is relative then the path will be interpreted relative to the current
/// working directory.
pub fn exists<P>(path: P) -> Result<bool>
    where P: ToCString,
{
    File::current_dir().rel_exists(path)
}

/// Checks whether the file at `path` can be accessed with the specified mode.
///
/// Relative paths are interpreted relative to the current working directory.
pub fn can_access<P>(path: P, mode: AccessMode) -> Result<bool>
    where P: ToCString,
{
    File::current_dir().rel_can_access(path, mode)
}

/// Sets the length of the file at `path`.
pub fn set_len<P>(path: P, len: u64) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    try!(retry(|| truncate(&path, len as loff_t)));
    Ok(())
}

/// Creates a hard link to `old` at `new`.
///
/// If `old` is a symlink then it is not dereferenced. Relative paths are interpreted
/// relative to the current working directory.
pub fn link<P, Q>(old: P, new: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let old = try!(old.rmo_cstr(&mut buf1));
    let new = try!(new.rmo_cstr(&mut buf2));
    rv!(linkat(AT_FDCWD, &old, AT_FDCWD, &new, 0))
}

/// Changes the access and modification times of the file specified by `path`.
///
/// Relative paths are interpreted relative to the current working directory. If `path` is
/// a symlink, then this changes the times of the destination.
pub fn set_times<P>(path: P, access: TimeChange, modification: TimeChange) -> Result
    where P: ToCString,
{
    File::current_dir().rel_set_times(path, access, modification)
}

/// Changes the access and modification times of the file specified by `path`.
///
/// Relative paths are interpreted relative to the current working directory. If `path` is
/// a symlink, then this changes the times of the symlink.
pub fn set_times_no_follow<P>(path: P, access: TimeChange,
                              modification: TimeChange) -> Result
    where P: ToCString,
{
    File::current_dir().rel_set_times_no_follow(path, access, modification)
}

/// Atomically exchanges the two files `one` and `two`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn exchange<P, Q>(one: P, two: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_exchange(one, two)
}

/// Renames `one` to `two`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `replace` is `false`, then the operation fails if `two` already exists.
pub fn rename<P, Q>(one: P, two: Q, replace: bool) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_rename(one, two, replace)
}

/// Creates the directory `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn create_dir<P>(path: P, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_dir(path, mode)
}

/// Removes the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `path` refers to a directory, then the directory has to be empty.
pub fn remove<P>(path: P) -> Result
    where P: ToCString,
{
    File::current_dir().rel_remove(path)
}

/// Creates a symlink from `link` to `target`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn symlink<P, Q>(target: P, link: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_symlink(target, link)
}

/// Reads the target of the symbolic link `link` into `buf`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn read_link_buf<P>(link: P, buf: &mut [u8]) -> Result<&mut NoNullStr>
    where P: ToCString,
{
    File::current_dir().rel_read_link_buf(link, buf)
}

/// Reads the target of the symbolic link `link`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn read_link<P>(link: P) -> Result<NoNullString<'static>>
    where P: ToCString,
{
    File::current_dir().rel_read_link(link)
}

/// Changes the owner of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn change_owner<P>(path: P, user: UserId, group: GroupId) -> Result
    where P: ToCString,
{
    File::current_dir().rel_change_owner(path, user, group)
}

/// Changes the owner of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.  If
/// `path` refers to a symlink, then this changes the owner of the symlink itself.
pub fn change_owner_no_follow<P>(path: P, user: UserId, group: GroupId) -> Result
    where P: ToCString,
{
    File::current_dir().rel_change_owner_no_follow(path, user, group)
}

/// Change the mode of the file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn change_mode<P>(path: P, mode: Mode) -> Result
    where P: ToCString,
{
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
pub fn create_file<P>(path: P, ty: Type, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_file(path, ty, mode)
}

/// Creates a device special file at `path`.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn create_device<P>(path: P, dev: Device, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_device(path, dev, mode)
}

/// Sets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn set_attr<P, S, V>(path: P, name: S, val: V) -> Result
    where P: ToCString, S: ToCString, V: AsRef<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(setxattr(&path, &name, val.as_ref(), 0))
}

/// Sets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn set_attr_no_follow<P, S, V>(path: P, name: S, val: V) -> Result
    where P: ToCString, S: ToCString, V: AsRef<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(lsetxattr(&path, &name, val.as_ref(), 0))
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn get_attr_buf<P, S, V>(path: P, name: S, mut val: V) -> Result<usize>
    where P: ToCString, S: ToCString, V: AsMut<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(getxattr(&path, &name, val.as_mut()), -> usize)
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn get_attr_no_follow_buf<P, S, V>(path: P, name: S, mut val: V) -> Result<usize>
    where P: ToCString, S: ToCString, V: AsMut<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(lgetxattr(&path, &name, val.as_mut()), -> usize)
}

fn get_attr_common<F>(mut f: F) -> Result<SVec<u8>>
    where F: FnMut(&mut [u8]) -> ssize_t,
{
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
                Err(error::RangeError) => { },
                Err(e) => return Err(e),
            }
        }
    }
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn get_attr<P, S>(path: P, name: S) -> Result<SVec<u8>>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    get_attr_common(|buf| getxattr(&path, &name, buf))
}

/// Gets an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn get_attr_no_follow<P, S>(path: P, name: S) -> Result<SVec<u8>>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    get_attr_common(|buf| lgetxattr(&path, &name, buf))
}

/// Removes an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn remove_attr<P, S>(path: P, name: S) -> Result
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(removexattr(&path, &name))
}

/// Removes an attribute of a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn remove_attr_no_follow<P, S>(path: P, name: S) -> Result
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf1));
    let name = try!(name.rmo_cstr(&mut buf2));
    rv!(lremovexattr(&path, &name))
}

/// Returns the buffer size required in a `list_attr_buf` call.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr_size<P>(path: P) -> Result<usize>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    rv!(listxattr(&path, &mut []), -> usize)
}

/// Returns the buffer size required in a `list_attr_buf_no_follow` call.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_size_no_follow<P>(path: P) -> Result<usize>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    rv!(llistxattr(&path, &mut []), -> usize)
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr_buf<'a, P>(path: P, buf: &'a mut [u8]) -> Result<ListAttrIter<'a>>
    where P: ToCString,
{
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut pbuf));
    let len = try!(rv!(listxattr(&path, buf), -> usize));
    Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_buf_no_follow<'a, P>(path: P,
                                      buf: &'a mut [u8]) -> Result<ListAttrIter<'a>>
    where P: ToCString,
{
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut pbuf));
    let len = try!(rv!(llistxattr(&path, buf), -> usize));
    Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
}

fn list_attr_common<F>(mut f: F) -> Result<ListAttrIterator>
    where F: FnMut(&mut [u8]) -> ssize_t,
{
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
                Err(error::RangeError) => { },
                Err(e) => return Err(e),
            }
        }
    }
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory.
pub fn list_attr<P>(path: P) -> Result<ListAttrIterator>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    list_attr_common(|buf| listxattr(&path, buf))
}

/// Returns an iterator over the attributes in a file.
///
/// Relative paths will be interpreted relative to the current working directory. If
/// `path` is a symbolic link, then the attribute of the symbolic link is set.
pub fn list_attr_no_follow<P>(path: P) -> Result<ListAttrIterator>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(path.rmo_cstr(&mut buf));
    list_attr_common(|buf| llistxattr(&path, buf))
}

/// An opened file in a file system.
#[derive(Eq)]
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
    pub fn open_read<P>(path: P) -> Result<File>
        where P: ToCString,
    {
        File::current_dir().rel_open_read(path)
    }

    /// Open the file at path `path` with the specified flags.
    pub fn open<P>(path: P, flags: Flags) -> Result<File>
        where P: ToCString,
    {
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
        try!(rv!(fstatat(self.fd, CStr::empty(), &mut stat, AT_EMPTY_PATH)));
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
    pub fn advise(&self, from: u64, to: Option<u64>, advice: Advice) -> Result {
        let len = match to {
            Some(e) => {
                assert!(e > from);
                e - from
            },
            _ => 0,
        };
        let ret = fadvise(self.fd, from as loff_t, len as loff_t, advice.to_c_int());
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
    pub fn link<P>(&self, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(linkat(self.fd, CStr::empty(), AT_FDCWD, &path, AT_EMPTY_PATH))
    }

    /// Creates a hard link to this file relative to a directory.
    ///
    /// Relative paths are interpreted relative to the directory `dir`.
    pub fn link_rel_to<P>(&self, dir: &File, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(linkat(self.fd, CStr::empty(), dir.fd, &path, AT_EMPTY_PATH))
    }

    /// Changes the access and modification times of this file.
    pub fn set_times(&self, access: TimeChange, modification: TimeChange) -> Result {
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, None, &times, 0))
    }

    /// Returns the path of the file that was used to open this file.
    pub fn filename_buf<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut NoNullStr> {
        // enough space for "/proc/self/fd/-{u64::MAX}\0"
        let mut proc_buf = [0; 36];
        let _ = write!(&mut proc_buf[..], "/proc/self/fd/{}", self.fd);
        // FIXME: not actually correct
        let cstr = proc_buf.as_cstr().unwrap();
        let len = try!(rv!(readlinkat(self.fd, cstr, buf), -> usize));
        Ok(buf[..len].as_mut_no_null_str().unwrap())
    }

    /// Returns the path of the file that was used to open this file.
    pub fn filename(&self) -> Result<NoNullString<'static>> {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        self.filename_buf(&mut buf).chain(|f| f.to_owned())
    }

    /// Changes the owner of this file.
    pub fn change_owner(&self, user: UserId, group: GroupId) -> Result {
        rv!(fchownat(self.fd, CStr::empty(), user, group, AT_EMPTY_PATH))
    }

    /// Changes the mode of this file.
    pub fn change_mode(&self, mode: Mode) -> Result {
        rv!(fchmod(self.fd, mode_to_int(mode)))
    }

    /// Initiates readahead of the specified range.
    pub fn readahead<R>(&self, range: R) -> Result
        where R: BoundedRange<u64>
    {
        let range = range.to_range();
        rv!(readahead(self.fd, range.start as loff_t, (range.end - range.start) as size_t))
    }

    /// Reserves the specified range in the file system.
    ///
    /// Further writes in the specified range are guaranteed not to fail because of a lack
    /// of storage capacity.
    pub fn reserve<R>(&self, range: R) -> Result
        where R: BoundedRange<u64>
    {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_KEEP_SIZE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }

    /// Creates a hole in the specified range.
    pub fn create_hole<R>(&self, range: R) -> Result
        where R: BoundedRange<u64>
    {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE,
                      range.start as loff_t, (range.end - range.start) as loff_t))
    }

    /// Removes the specified range from the file and closes the gap.
    ///
    /// The range must probably begin and end at a multiple of the block size but this
    /// depends on the filesystem. This function cannot be used if the range reaches the
    /// end of the file. Use `set_len` for this purpose.
    pub fn collapse<R>(&self, range: R) -> Result
        where R: BoundedRange<u64>
    {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_COLLAPSE_RANGE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }
    
    /// Zeroes the specified range in the file.
    /// 
    /// This can be more efficient than manually writing zeroes.
    pub fn zero<R>(&self, range: R) -> Result
        where R: BoundedRange<u64>
    {
        let range = range.to_range();
        rv!(fallocate(self.fd, FALLOC_FL_ZERO_RANGE, range.start as loff_t,
                      (range.end - range.start) as loff_t))
    }

    /// Sets an attribute of this file.
    pub fn set_attr<S, V>(&self, name: S, val: V) -> Result
        where S: ToCString, V: AsRef<[u8]>,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name = try!(name.rmo_cstr(&mut buf));
        rv!(fsetxattr(self.fd, &name, val.as_ref(), 0))
    }

    /// Gets an attribute of this file.
    pub fn get_attr_buf<S, V>(&self, name: S, mut val: V) -> Result<usize>
        where S: ToCString, V: AsMut<[u8]>,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name = try!(name.rmo_cstr(&mut buf));
        rv!(fgetxattr(self.fd, &name, val.as_mut()), -> usize)
    }

    /// Gets an attribute of this file.
    pub fn get_attr<S>(&self, name: S) -> Result<SVec<u8>>
        where S: ToCString,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name = try!(name.rmo_cstr(&mut buf));
        get_attr_common(|buf| fgetxattr(self.fd, &name, buf))
    }

    /// Removes an attribute of this file.
    pub fn remove_attr<S>(&self, name: S) -> Result
        where S: ToCString,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name = try!(name.rmo_cstr(&mut buf));
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
    pub fn rel_open_read<P>(&self, path: P) -> Result<File>
        where P: ToCString,
    {
        self.rel_open(path, Flags::new())
    }

    /// Open the file at path `path` with the specified flags.
    ///
    /// If `path` is relative, the `self` must be a directory and the `path` will be
    /// interpreted relative to `self`.
    pub fn rel_open<P>(&self, path: P, flags: Flags) -> Result<File>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let mode = flags.mode().map(|m| mode_to_int(m)).unwrap_or(0);
        let fd = match retry(|| openat(self.fd, &path,
                                       flags_to_int(flags) | cty::O_LARGEFILE, mode)) {
            Ok(fd) => fd,
            // Due to a bug in the kernel, open returns WrongDeviceType instead of
            // NoSuchDevice.
            Err(error::WrongDeviceType) => return Err(error::NoSuchDevice),
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
    pub fn rel_info<P>(&self, path: P) -> Result<Info>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let mut stat = unsafe  { mem::zeroed() };
        try!(rv!(fstatat(self.fd, &path, &mut stat, 0)));
        Ok(info_from_stat(stat))
    }

    /// Returns information about the file specified by `path`.
    ///
    /// This returns information about the file at `path`, even if `path` is a symlink.
    /// If `path` is relative, then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_info_no_follow<P>(&self, path: P) -> Result<Info>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let mut stat = unsafe  { mem::zeroed() };
        try!(rv!(fstatat(self.fd, &path, &mut stat, AT_SYMLINK_NOFOLLOW)));
        Ok(info_from_stat(stat))
    }

    /// Returns whether the specified path points to an existing file.
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_exists<P>(&self, path: P) -> Result<bool>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let res = faccessat(self.fd, &path, 0);
        if res >= 0 {
            Ok(true)
        } else {
            let err = Errno(-res);
            match err {
                error::DoesNotExist => Ok(false),
                _ => Err(err),
            }
        }
    }

    /// Checks whether the file at `path` can be accessed with the specified mode.
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_can_access<P>(&self, path: P, mode: AccessMode) -> Result<bool>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let res = faccessat(self.fd, &path, access_mode_to_int(mode));
        if res >= 0 {
            Ok(true)
        } else {
            let err = Errno(-res);
            match err {
                error::AccessDenied => Ok(false),
                _ => Err(err),
            }
        }
    }

    /// Changes the access and modification times of the file specified by `path`.
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the destination.
    pub fn rel_set_times<P>(&self, path: P, access: TimeChange,
                            modification: TimeChange) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, 0))
    }

    /// Changes the access and modification times of the file specified by `path`.
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the symlink.
    pub fn rel_set_times_no_follow<P>(&self, path: P, access: TimeChange,
                                      modification: TimeChange) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, AT_SYMLINK_NOFOLLOW))
    }

    /// Atomically exchanges the two files `one` and `two`.
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`.
    pub fn rel_exchange<P, Q>(&self, one: P, two: Q) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let one = try!(one.rmo_cstr(&mut buf1));
        let two = try!(two.rmo_cstr(&mut buf2));
        rv!(renameat(self.fd, &one, self.fd, &two, RENAME_EXCHANGE))
    }

    /// Renames `one` to `two`.
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`. If `replace` is `false`, then the
    /// operation fails if `two` already exists.
    pub fn rel_rename<P, Q>(&self, one: P, two: Q, replace: bool) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let one = try!(one.rmo_cstr(&mut buf1));
        let two = try!(two.rmo_cstr(&mut buf2));
        let flag = if replace { 0 } else { RENAME_NOREPLACE };
        rv!(renameat(self.fd, &one, self.fd, &two, flag))
    }

    /// Creates the directory `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`.
    pub fn rel_create_dir<P>(&self, path: P, mode: Mode) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(mkdirat(self.fd, &path, mode_to_int(mode)))
    }

    /// Removes the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`. If `path` refers to a directory, then the
    /// directory has to be empty.
    pub fn rel_remove<P>(&self, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        let mut ret = unlinkat(self.fd, &path, 0);
        if Errno(-ret) == error::IsADirectory {
            ret = unlinkat(self.fd, &path, AT_REMOVEDIR);
        }
        rv!(ret)
    }

    /// Creates a symlink from `link` to `target`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_symlink<P, Q>(&self, target: P, link: Q) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let target = try!(target.rmo_cstr(&mut buf1));
        let link = try!(link.rmo_cstr(&mut buf2));
        rv!(symlinkat(&target, self.fd, &link))
    }

    /// Reads the target of the symbolic link `link` into `buf`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_read_link_buf<'a, P>(&self, link: P,
                                    buf: &'a mut [u8]) -> Result<&'a mut NoNullStr>
        where P: ToCString,
    {
        let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let link = try!(link.rmo_cstr(&mut pbuf));
        let len = try!(rv!(readlinkat(self.fd, &link, buf), -> usize));
        Ok(unsafe { NoNullStr::from_bytes_unchecked_mut(&mut buf[..len]) })
    }

    /// Reads the target of the symbolic link `link`.
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_read_link<P>(&self, link: P) -> Result<NoNullString<'static>>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        self.rel_read_link_buf(link, &mut buf).chain(|f| f.to_owned())
    }

    /// Changes the owner of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_owner<P>(&self, path: P, user: UserId, group: GroupId) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, 0))
    }

    /// Changes the owner of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`. If `path` refers to a symlink, then this changes
    /// the owner of the symlink itself.
    pub fn rel_change_owner_no_follow<P>(&self, path: P, user: UserId,
                                         group: GroupId) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, AT_SYMLINK_NOFOLLOW))
    }

    /// Change the mode of the file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_mode<P>(&self, path: P, mode: Mode) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
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
    pub fn rel_create_file<P>(&self, path: P, ty: Type, mode: Mode) -> Result
        where P: ToCString,
    {
        match ty {
            Type::File | Type::FIFO | Type::Socket => { },
            _ => return Err(error::InvalidArgument),
        }
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), 0))
    }

    /// Creates a device special file at `path`.
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_create_device<P>(&self, path: P, dev: Device, mode: Mode) -> Result
        where P: ToCString,
    {
        let ty = match dev.ty() {
            DeviceType::Character => Type::CharDevice,
            DeviceType::Block     => Type::BlockDevice,
        };
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(path.rmo_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), dev.id()))
    }
}

impl Read for File {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        File::scatter_read(self, buf)
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
    fn unwrap(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> File {
        File { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> File {
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
#[derive(Copy, Eq)]
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
#[derive(Copy, Eq)]
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
    type Item = &'a ByteStr;

    fn next(&mut self) -> Option<&'a ByteStr> {
        if self.pos == self.buf.len() {
            return None;
        }
        let buf = &self.buf[self.pos..];
        let len = memchr(buf, 0).unwrap();
        self.pos += len + 1;
        Some(buf[..len].as_byte_str())
    }
}

pub struct ListAttrIterator {
    buf: SVec<u8>,
    pos: usize,
}

impl Iterator for ListAttrIterator {
    type Item = ByteString<'static>;

    fn next(&mut self) -> Option<ByteString<'static>> {
        if self.pos == self.buf.len() {
            return None;
        }
        let buf = &self.buf[self.pos..];
        let len = memchr(buf, 0).unwrap();
        self.pos += len + 1;
        Some(buf[..len].as_byte_str().to_owned().unwrap())
    }
}
