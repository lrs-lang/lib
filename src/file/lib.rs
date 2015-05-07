// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_file"]
#![crate_type = "lib"]
#![feature(plugin, no_std, negate_unsigned, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core      as core;
extern crate lrs_base      as base;
extern crate lrs_io        as io;
extern crate lrs_cty       as cty;
extern crate lrs_int       as int;
extern crate lrs_syscall   as syscall;
extern crate lrs_str_one   as str_one;
extern crate lrs_str_two   as str_two;
extern crate lrs_str_three as str_three;
extern crate lrs_arch_fns  as arch_fns;
extern crate lrs_rv        as rv;
extern crate lrs_fmt       as fmt;
extern crate lrs_vec       as vec;
extern crate lrs_rmo       as rmo;
extern crate lrs_parse     as parse;
extern crate lrs_fd        as fd;
extern crate lrs_alloc     as alloc;
extern crate lrs_dev       as dev;
extern crate lrs_fs        as fs;
extern crate lrs_time_base as time_base;

#[prelude_import] use base::prelude::*;
mod lrs { pub use vec::lrs::*; pub use {cty}; }

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
use rmo::{Rmo, ToOwned};
use alloc::{FbHeap};
use io::{Write};

use time_base::{Time, time_to_timespec};

use fs::info::{FileSystemInfo, from_statfs};

use dev::{Device, DeviceType};

use flags::{Flags, Mode, AccessMode, access_mode_to_int, flags_from_int, flags_to_int,
            mode_to_int};
use info::{Info, info_from_stat, Type, file_type_to_mode};

pub mod flags;
pub mod info;

/// Retrieves information about a file.
///
/// [argument, path]
/// The path of the file whose information is to be retrieved.
///
/// [return_value]
/// The file information.
///
/// = Remarks
///
/// If the path refers to a symbolic link, then it will recursively be resolved and the
/// information of the first non-link target will be returned.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:lrs::file::info_no_follow
/// * link:lrs::file::File::rel_info
/// * link:man:stat(2)
pub fn _info<P>(path: P) -> Result<Info>
    where P: ToCString,
{
    File::current_dir().rel_info(path)
}

/// Retrieves information about a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose information is to be retrieved.
///
/// [return_value]
/// The file information.
///
/// = Remarks
///
/// This function does not follow symbolic links and always returns information about the
/// file specified by the path.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:lrs::file::info
/// * link:lrs::file::File::rel_info_no_follow
/// * link:man:stat(2)
pub fn info_no_follow<P>(path: P) -> Result<Info>
    where P: ToCString,
{
    File::current_dir().rel_info_no_follow(path)
}

/// Checks whether a file exists.
///
/// [argument, path]
/// The path to the file to be checked.
///
/// [return_value]
/// Returns `true` if the file exists, `false` otherwise.
///
/// = Remarks
///
/// If the path refers to a symbolic link, then the existence of the symbolic link is
/// checked.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:faccessat(2)
/// * link:lrs::file::File::rel_exists
pub fn exists<P>(path: P) -> Result<bool>
    where P: ToCString,
{
    File::current_dir().rel_exists(path)
}

/// Checks whether a file can be accessed with a certain mode.
///
/// [argument, path]
/// The path of the file to be checked.
///
/// [argument, mode]
/// The mode we want to access the file with.
///
/// [return_value]
/// Returns `true` if the file can be accessed with the specified mode, `false` otherwise.
///
/// = Remarks
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:faccessat(2)
/// * link:lrs::file::File::rel_can_access
pub fn can_access<P>(path: P, mode: AccessMode) -> Result<bool>
    where P: ToCString,
{
    File::current_dir().rel_can_access(path, mode)
}

/// Truncates a file to a certain length.
///
/// [argument, path]
/// The path of the file to be truncated.
///
/// [argument, len]
/// The length we want to truncate the file to.
///
/// = Remarks
///
/// The new length can be larger than the old length.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:truncate(2)
/// * link:lrs::file::File::set_len
pub fn set_len<P>(path: P, len: u64) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    try!(retry(|| truncate(&path, len as loff_t)));
    Ok(())
}

/// Creates a hard link.
///
/// [argument, source]
/// The path of the file we want to link to.
///
/// [argument, link]
/// The path of the new hard link.
///
/// = Remarks
///
/// If `source` refers to a symbolic link, the new link will refer to the symbolic link
/// and not the target of the symbolic link.
///
/// `source` and `link` have to be located in the same mount point.
///
/// If the paths are relative, they will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:link(2)
pub fn link<P, Q>(source: P, link: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let old: Rmo<_, FbHeap> = try!(source.rmo_cstr(&mut buf1));
    let new: Rmo<_, FbHeap> = try!(link.rmo_cstr(&mut buf2));
    rv!(linkat(AT_FDCWD, &old, AT_FDCWD, &new, 0))
}

/// Changes the access and modification times of a file.
///
/// [argument, path]
/// The path of the file we want to modify.
///
/// [argument, access]
/// The access time change to be performed.
///
/// [argument, modification]
/// The modification time change to be performed.
///
/// = Remarks
///
/// If `path` refers to a symbolic link, the symbolic link will recursively be resolved
/// and the times of the first non-link target will be modified.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:utimensat(2)
/// * link:lrs::file::File::rel_set_times
/// * link:lrs::file::set_times_no_follow
pub fn set_times<P>(path: P, access: TimeChange, modification: TimeChange) -> Result
    where P: ToCString,
{
    File::current_dir().rel_set_times(path, access, modification)
}

/// Changes the access and modification times of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file we want to modify.
///
/// [argument, access]
/// The access time change to be performed.
///
/// [argument, modification]
/// The modification time change to be performed.
///
/// = Remarks
///
/// If `path` refers to a symbolic link, the times of the symbolic link itself will be
/// modified.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:utimensat(2)
/// * link:lrs::file::File::rel_set_times
/// * link:lrs::file::set_times_no_follow
pub fn set_times_no_follow<P>(path: P, access: TimeChange,
                              modification: TimeChange) -> Result
    where P: ToCString,
{
    File::current_dir().rel_set_times_no_follow(path, access, modification)
}

/// Atomically exchanges two files.
///
/// [argument, one]
/// File one.
///
/// [argument, two]
/// File two.
///
/// = Remarks
///
/// The files can have different type. For example, one could refer to a directory and the
/// other one could refer to a symbolic link.
///
/// If the paths are relative, they will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:renameat2(2)
/// * link:lrs::file::File::rel_exchange
pub fn exchange<P, Q>(one: P, two: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_exchange(one, two)
}

/// Renames a file.
///
/// [argument, from]
/// The path of the file to be renamed.
///
/// [argument, to]
/// The new name of the file.
///
/// [argument, replace]
/// Whether `to` is replaced if it already exists.
///
/// = Remarks
///
/// If the paths are relative, they will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:renameat2(2)
/// * link:lrs::file::File::rel_rename
pub fn rename<P, Q>(from: P, to: Q, replace: bool) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_rename(from, to, replace)
}

/// Creates a directory.
///
/// [argument, path]
/// The path of the new directory.
///
/// [argument, mode]
/// The mode of the new directory.
///
/// = Remarks
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:mkdirat(2)
/// * link:lrs::file::File::rel_create_dir
pub fn create_dir<P>(path: P, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_dir(path, mode)
}

/// Removes a file.
///
/// [argument, path]
/// The path of the file to be removed.
///
/// = Remarks
///
/// If the path refers to a directory, the directory has to be empty.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:unlinkat(2)
/// * link:lrs::file::File::rel_remove
pub fn remove<P>(path: P) -> Result
    where P: ToCString,
{
    File::current_dir().rel_remove(path)
}

/// Creates a symbolic link.
///
/// [argument, source]
/// The path to be linked to.
///
/// [argument, link]
/// The path of the new link.
///
/// = Remarks
///
/// If the paths are relative, they will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:symlinkat(2)
/// * link:lrs::file::File::rel_symlink
pub fn symlink<P, Q>(source: P, link: Q) -> Result
    where P: ToCString, Q: ToCString,
{
    File::current_dir().rel_symlink(source, link)
}

/// Retrieves the target of a symbolic link.
///
/// [argument, link]
/// The link whose target is to be retrieved.
///
/// [argument, buf]
/// The buffer where the target will be stored in.
///
/// [return_value]
/// Returns the target of the link.
///
/// = Remarks
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:readlinkat(2)
/// * link:lrs::file::read_link
/// * link:lrs::file::File::rel_read_link_buf
pub fn read_link_buf<P>(link: P, buf: &mut [u8]) -> Result<&mut NoNullStr>
    where P: ToCString,
{
    File::current_dir().rel_read_link_buf(link, buf)
}

/// Retrieves the target of a symbolic link.
///
/// [argument, link]
/// The link whose target is to be retrieved.
///
/// [return_value]
/// Returns the target of the link.
///
/// = Remarks
///
/// The memory for the target will be allocated.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:readlinkat(2)
/// * link:lrs::file::read_link_buf
/// * link:lrs::file::File::rel_read_link
pub fn read_link<P>(link: P) -> Result<NoNullString<'static>>
    where P: ToCString,
{
    File::current_dir().rel_read_link(link)
}

/// Changes the owner of a file.
///
/// [argument, path]
/// The path of the file whose owner will be changed.
///
/// [argument, user]
/// The user id of the new owner.
///
/// [argument, group]
/// The group id of the new owner.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// owner of the first non-link target will be changed.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:fchownat(2)
/// * link:lrs::file::File::rel_change_owner
pub fn change_owner<P>(path: P, user: UserId, group: GroupId) -> Result
    where P: ToCString,
{
    File::current_dir().rel_change_owner(path, user, group)
}

/// Changes the owner of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose owner will be changed.
///
/// [argument, user]
/// The user id of the new owner.
///
/// [argument, group]
/// The group id of the new owner.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the owner of the link will be changed.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:fchownat(2)
/// * link:lrs::file::File::rel_change_owner_no_follow
pub fn change_owner_no_follow<P>(path: P, user: UserId, group: GroupId) -> Result
    where P: ToCString,
{
    File::current_dir().rel_change_owner_no_follow(path, user, group)
}

/// Change the mode of a file.
///
/// [argument, path]
/// The path of the file whose mode will be changed.
///
/// [argument, mode]
/// The new mode of the file.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// mode of the first non-link target will be changed.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:fchmodat(2)
/// * link:lrs::file::File::rel_change_mode
pub fn change_mode<P>(path: P, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_change_mode(path, mode)
}

/// Creates a file.
///
/// [argument, path]
/// The path at which the file will be created.
///
/// [argument, ty]
/// The type of the new file.
///
/// [argument, mode]
/// The mode of the new file.
///
/// = Remarks
///
/// The type must be either `File`, `FIFO`, or `Socket`.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:mknodat(2)
/// * link:lrs::file::create_device
/// * link:lrs::file::File::rel_create_file
pub fn create_file<P>(path: P, ty: Type, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_file(path, ty, mode)
}

/// Creates a device special file.
///
/// [argument, path]
/// The path at which the file will be created.
///
/// [argument, dev]
/// The device special file to create.
///
/// [argument, mode]
/// The mode of the new file.
///
/// = Remarks
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man::mknodat(2)
/// * link:lrs::file::File::rel_create_device
pub fn create_device<P>(path: P, dev: Device, mode: Mode) -> Result
    where P: ToCString,
{
    File::current_dir().rel_create_device(path, dev, mode)
}

/// Sets an attribute of a file.
///
/// [argument, path]
/// The path of the file whose attribute to change.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The new value of the attribute.
///
/// = Remarks
///
/// If the attribute does not exist, it will be created. If the attribute exists, it will
/// be overwritten.
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// attribute of the first non-link target will be set.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man::setxattr(2)
/// * link:lrs::file::set_attr_no_follow
/// * link:lrs::file::File::set_attr
pub fn set_attr<P, S, V>(path: P, name: S, val: V) -> Result
    where P: ToCString, S: ToCString, V: AsRef<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(setxattr(&path, &name, val.as_ref(), 0))
}

/// Sets an attribute of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose attribute to change.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The new value of the attribute.
///
/// = Remarks
///
/// If the attribute does not exist, it will be created. If the attribute exists, it will
/// be overwritten.
///
/// If the path refers to a symbolic link, the attribute of the link will be set.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man::setxattr(2)
/// * link:lrs::file::set_attr
/// * link:lrs::file::File::set_attr
pub fn set_attr_no_follow<P, S, V>(path: P, name: S, val: V) -> Result
    where P: ToCString, S: ToCString, V: AsRef<[u8]>,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(lsetxattr(&path, &name, val.as_ref(), 0))
}

/// Retrieves an attribute of a file.
///
/// [argument, path]
/// The path of the file whose attribute we want to retrieve.
///
/// [argument, name]
/// The name of the attribute to retrieve.
///
/// [argument, buf]
/// The buffer in which the attribute will be stored.
///
/// [return_value]
/// The number of bytes stored in the buffer.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// attribute of the first non-link target will be retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:getxattr(2)
/// * link:lrs::file::get_attr_no_follow_buf
/// * link:lrs::file::get_attr
/// * link:lrs::file::File::get_attr_buf
pub fn get_attr_buf<P, S>(path: P, name: S, buf: &mut [u8]) -> Result<usize>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(getxattr(&path, &name, buf), -> usize)
}

/// Retrieves an attribute of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose attribute we want to retrieve.
///
/// [argument, name]
/// The name of the attribute to retrieve.
///
/// [argument, buf]
/// The buffer in which the attribute will be stored.
///
/// [return_value]
/// The number of bytes stored in the buffer.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the attribute of the file  will be retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:getxattr(2)
/// * link:lrs::file::get_attr_buf
/// * link:lrs::file::get_attr_no_follow
/// * link:lrs::file::File::get_attr_buf
pub fn get_attr_no_follow_buf<P, S, V>(path: P, name: S, buf: &mut [u8]) -> Result<usize>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(lgetxattr(&path, &name, buf), -> usize)
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

/// Retrieves an attribute of a file.
///
/// [argument, path]
/// The path of the file whose attribute we want to retrieve.
///
/// [argument, name]
/// The name of the attribute to retrieve.
///
/// [return_value]
/// The attribute.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// attribute of the first non-link target will be retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:getxattr(2)
/// * link:lrs::file::get_attr_no_follow
/// * link:lrs::file::get_attr_buf
/// * link:lrs::file::File::get_attr
pub fn get_attr<P, S>(path: P, name: S) -> Result<SVec<u8>>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    get_attr_common(|buf| getxattr(&path, &name, buf))
}

/// Retrieves an attribute of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose attribute we want to retrieve.
///
/// [argument, name]
/// The name of the attribute to retrieve.
///
/// [return_value]
/// The attribute.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the attribute of the link will be retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:getxattr(2)
/// * link:lrs::file::get_attr_no_follow_buf
/// * link:lrs::file::get_attr
/// * link:lrs::file::File::get_attr
pub fn get_attr_no_follow<P, S>(path: P, name: S) -> Result<SVec<u8>>
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    get_attr_common(|buf| lgetxattr(&path, &name, buf))
}

/// Removes an attribute of a file.
///
/// [argument, path]
/// The path of the file whose argument we want to remove.
///
/// [argument, name]
/// The name of the argument.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// attribute of the first non-link target will be removed.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:removexattr(2)
/// * link:lrs::file::remove_attr_no_follow
/// * link:lrs::file::File::remove_attr
pub fn remove_attr<P, S>(path: P, name: S) -> Result
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(removexattr(&path, &name))
}

/// Removes an attribute of a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose argument we want to remove.
///
/// [argument, name]
/// The name of the argument.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the attribute of the link will be removed.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:removexattr(2)
/// * link:lrs::file::remove_attr
/// * link:lrs::file::File::remove_attr
pub fn remove_attr_no_follow<P, S>(path: P, name: S) -> Result
    where P: ToCString, S: ToCString,
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [u8; 128] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf1));
    let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf2));
    rv!(lremovexattr(&path, &name))
}

/// Retrieves the buffer size required for all attributes in a file.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [return_value]
/// Returns the number of bytes required.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the link will be recursively resolved and the
/// required buffer size of the first non-link target will be retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * link:lrs::file::list_attr_size_no_follow
/// * link:lrs::file::list_attr
/// * link:lrs::file::File::list_attr_size
pub fn list_attr_size<P>(path: P) -> Result<usize>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    rv!(listxattr(&path, &mut []), -> usize)
}

/// Retrieves the buffer size required for all attributes in a file without following
/// symbolic links.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [return_value]
/// Returns the number of bytes required.
///
/// = Remarks
///
/// If the path refers to a symbolic link, the required buffer size for the link will be
/// retrieved.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * link:lrs::file::list_attr_size
/// * link:lrs::file::list_attr_no_follow
/// * link:lrs::file::File::list_attr_size
pub fn list_attr_size_no_follow<P>(path: P) -> Result<usize>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    rv!(llistxattr(&path, &mut []), -> usize)
}

/// Creates an iterator over all attributes in a file.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [argument, buf]
/// The buffer in which the attributes will be stored.
///
/// [return_value]
/// Returns an iterator over the attributes in the file.
///
/// = Remarks
///
/// :list_attr_size: link:lrs::file::list_attr_size
///
/// Use {list_attr_size}[list_attr_size] to get the required buffer size. It is an error
/// for `buf` to have length `0`.
///
/// If the path refers to a symbolic link, the link will be recursively resolved and an
/// Iterator over the attributes of the first non-link target is returned.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * {list_attr_size}
/// * link:lrs::file::list_attr
/// * link:lrs::file::list_attr_buf_no_follow
/// * link:lrs::file::File::list_attr_buf
pub fn list_attr_buf<'a, P>(path: P, buf: &'a mut [u8]) -> Result<ListAttrIter<'a>>
    where P: ToCString,
{
    if buf.len() == 0  { return Err(error::InvalidArgument); }
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut pbuf));
    let len = try!(rv!(listxattr(&path, buf), -> usize));
    Ok(ListAttrIter { buf: &buf[..len], pos: 0 })
}

/// Creates an iterator over all attributes in a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [argument, buf]
/// The buffer in which the attributes will be stored.
///
/// [return_value]
/// Returns an iterator over the attributes in the file.
///
/// = Remarks
///
/// :list_attr_size_no_follow: link:lrs::file::list_attr_size_no_follow
///
/// Use {list_attr_size_no_follow}[list_attr_size_no_follow] to get the required buffer
/// size. It is an error for `buf` to have length `0`.
///
/// If the path refers to a symbolic link, an Iterator over the attributes of the link is
/// returned.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * {list_attr_size_no_follow}
/// * link:lrs::file::list_attr_no_follow
/// * link:lrs::file::list_attr_buf
/// * link:lrs::file::File::list_attr_buf
pub fn list_attr_buf_no_follow<'a, P>(path: P,
                                      buf: &'a mut [u8]) -> Result<ListAttrIter<'a>>
    where P: ToCString,
{
    if buf.len() == 0  { return Err(error::InvalidArgument); }
    let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut pbuf));
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

/// Creates an iterator over all attributes in a file.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [return_value]
/// Returns an iterator over the attributes in the file.
///
/// = Remarks
///
/// :list_attr_size: link:lrs::file::list_attr_size
///
/// Use {list_attr_size}[list_attr_size] to get the required buffer size. It is an error
/// for `buf` to have length `0`.
///
/// If the path refers to a symbolic link, the link will be recursively resolved and an
/// Iterator over the attributes of the first non-link target is returned.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * {list_attr_size}
/// * link:lrs::file::list_attr_buf
/// * link:lrs::file::list_attr_no_follow
/// * link:lrs::file::File::list_attr
pub fn list_attr<P>(path: P) -> Result<ListAttrIterator>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    list_attr_common(|buf| listxattr(&path, buf))
}

/// Creates an iterator over all attributes in a file without following symbolic links.
///
/// [argument, path]
/// The path of the file whose attributes we're interested in.
///
/// [return_value]
/// Returns an iterator over the attributes in the file.
///
/// = Remarks
///
/// :list_attr_size_no_follow: link:lrs::file::list_attr_size_no_follow
///
/// Use {list_attr_size_no_follow}[list_attr_size_no_follow] to get the required buffer
/// size. It is an error for `buf` to have length `0`.
///
/// If the path refers to a symbolic link, an Iterator over the attributes of the link is
/// returned.
///
/// If the path is relative, it will be interpreted relative to the current working
/// directory.
///
/// = See also
///
/// * link:man:listxattr(2)
/// * {list_attr_size_no_follow}
/// * link:lrs::file::list_attr_no_follow
/// * link:lrs::file::list_attr
/// * link:lrs::file::File::list_attr
pub fn list_attr_no_follow<P>(path: P) -> Result<ListAttrIterator>
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    ///
    /// [return_value]
    /// Returns an invalid file.
    ///
    /// = Remarks
    ///
    /// This is equivalent to calling `File::from_borrowed(-1)`. This can be useful when a
    /// function which always returns a `File` or an object containing a `File` has to
    /// signal an error condition.
    pub fn invalid() -> File {
        File { fd: -1, owned: false }
    }

    /// Creates a file that points to the current working directory.
    ///
    /// [return_value]
    /// Returns a file that points to the current working directory.
    ///
    /// = Remarks
    ///
    /// :setcwd: link:lrs::process::set_cwd
    /// :open: link:lrs::file::File::open
    ///
    /// This call does not actually open a directory. The returned `File` is thus affected
    /// by changes of the global current working directory via {setcwd} . If you require a
    /// pointer to the current directory that is not affected by changes to the current
    /// working directory of the process, use the following code:
    ///
    /// ----
    /// let mut flags = Flags::new();
    /// flags.set_path_fd(true);
    /// let file = try!(File::open(".", flags);
    /// ----
    ///
    /// This call is mostly useful when working with interfaces that require an argument
    /// which points to a directory. For example, {open}[`File::open`] is implemented like
    /// this:
    ///
    /// ----
    /// pub fn open_read<P>(path: P) -> Result<File>
    ///     where P: ToCString,
    /// {
    ///     File::current_dir().rel_open_read(path)
    /// }
    /// ----
    ///
    /// = See also
    ///
    /// * {setcwd}
    pub fn current_dir() -> File {
        File { fd: AT_FDCWD, owned: false }
    }

    /// Opens a file in read-only mode.
    ///
    /// [argument, path]
    /// The path of the file to be opened.
    ///
    /// [return_value]
    /// Returns the opened file.
    ///
    /// = Remarks
    ///
    /// If the path refers to a symbolic link, the link is recursively resolved and the
    /// first non-link target is opened.
    ///
    /// If the path is relative, it is interpreted relative to the current working
    /// directory.
    ///
    /// This is equivalent to `File::open` with the default flags.
    ///
    /// = See also
    ///
    /// * link:open(2)
    /// * link:lrs::file::File::open
    pub fn open_read<P>(path: P) -> Result<File>
        where P: ToCString,
    {
        File::current_dir().rel_open_read(path)
    }

    /// Opens a file with custom flags.
    ///
    /// [argument, path]
    /// The path of the file to be opened.
    ///
    /// [argument, flags]
    /// The flags to be used when opening a file.
    ///
    /// [return_value]
    /// Return the opened file.
    ///
    /// = Remarks
    ///
    /// If the path refers to a symbolic link, the link is recursively resolved and the
    /// first non-link target is opened.
    ///
    /// If the path is relative, it is interpreted relative to the current working
    /// directory.
    ///
    /// = See also
    ///
    /// * link:open(2)
    /// * link:lrs::file::File::open_read
    pub fn open<P>(path: P, flags: Flags) -> Result<File>
        where P: ToCString,
    {
        File::current_dir().rel_open(path, flags)
    }

    /// Reads from the file.
    ///
    /// [argument, buf]
    /// The buffer that will be filled by the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:read(2)
    /// * link:lrs::file::File::read_at
    /// * link:lrs::file::File::scatter_read
    /// * link:lrs::file::File::scatter_read_at
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        retry(|| read(self.fd, buf)).map(|r| r as usize)
    }

    /// Writes to the file.
    ///
    /// [argument, buf]
    /// The buffer that will be written to the file.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:write(2)
    /// * link:lrs::file::File::write_at
    /// * link:lrs::file::File::gather_write
    /// * link:lrs::file::File::gather_write_at
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        retry(|| write(self.fd, buf)).map(|r| r as usize)
    }

    /// Closes the file.
    ///
    /// = Remarks
    ///
    /// If the file is not owned, an error is returned. If the file is owned and the
    /// kernel returns an error, the error is returned but the file descriptor contained
    /// in this object is nonetheless replaced by `-1`.
    ///
    /// = See also
    ///
    /// * link:man:close(2)
    pub fn close(&mut self) -> Result {
        if self.owned {
            let ret = close(self.fd);
            self.fd = -1;
            rv!(ret)
        } else {
            Err(error::InvalidArgument)
        }
    }

    /// Retrieves information about the file.
    ///
    /// [return_value]
    /// The retrieved information.
    ///
    /// = See also
    ///
    /// * link:man:fstatat(2)
    pub fn info(&self) -> Result<Info> {
        let mut stat = mem::zeroed();
        try!(rv!(fstatat(self.fd, CStr::empty(), &mut stat, AT_EMPTY_PATH)));
        Ok(info_from_stat(stat))
    }

    /// Changes the read/write position of the file.
    ///
    /// [argument, pos]
    /// The seek operation to be performed.
    ///
    /// [return_value]
    /// Returns the new position in the file.
    ///
    /// = See also
    ///
    /// * link:man:lseek(2)
    pub fn seek(&self, pos: Seek) -> Result<i64> {
        let ret = lseek(self.fd, pos.offset(), pos.whence());
        rv!(ret, -> i64)
    }

    /// Creates a new file referring to the same file description.
    ///
    /// [return_value]
    /// Returns the new file.
    ///
    /// = Remarks
    ///
    /// The `close on exec` flag will be set on the new file.
    ///
    /// The new file has its own file descriptor which refers to the same file
    /// description. This means that changing the `close on exec` flag on this file will
    /// not affect the other file and neither does closing this file. But writing,
    /// reading, seeking, etc. will affect the other file.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the `F_DUPFD_CLOEXEC` section therein.
    /// * link:man:open(2) and the description of *file descriptors* and *file
    ///   descriptions* therein.
    pub fn duplicate(&self) -> Result<File> {
        let new_fd = fcntl_dupfd_cloexec(self.fd, 0);
        if new_fd < 0 {
            Err(Errno(-new_fd as c_int))
        } else {
            Ok(File { fd: new_fd, owned: true })
        }
    }

    /// Retrieves the file status flags and access mode.
    ///
    /// [return_value]
    /// Returns the status flags and access mode.
    ///
    /// = Remarks
    ///
    /// The status flags and access mode are part of the file description, not the file
    /// descriptor. The status flags are
    ///
    /// * `bypass buffer`
    /// * `access time update`
    /// * `append`
    /// * `signal io`
    /// * `data synchronized`
    /// * `non blocking`
    /// * `synchronized`
    /// * `path fd`
    ///
    /// The access mode are
    ///
    /// * `readable`
    /// * `writable`
    ///
    /// The status of the other flags in unspecified.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_GETFL` therein.
    /// * link:lrs::file::File::set_status_flags
    pub fn get_status_flags(&self) -> Result<Flags> {
        let ret = fcntl_getfl(self.fd);
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(flags_from_int(ret))
        }
    }

    /// Sets the file status flags.
    ///
    /// [argument, flags]
    /// The modified flags.
    ///
    /// = Remarks
    ///
    /// Only the following flags can be changed
    ///
    /// * `append`
    /// * `signal io`
    /// * `bypass buffer`
    /// * `access time update`
    /// * `non blocking`
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_SETFL` therein.
    /// * link:lrs::file::File::get_status_flags
    pub fn set_status_flags(&self, flags: Flags) -> Result {
        let ret = fcntl_setfl(self.fd, flags_to_int(flags));
        rv!(ret)
    }

    /// Retrieves the status of the `close on exec` flag.
    ///
    /// [return_value]
    /// Returns whether the `close on exec` flag is set.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_GETFD` therein.
    /// * link:lrs::file::File::set_close_on_exec
    pub fn is_close_on_exec(&self) -> Result<bool> {
        let ret = fcntl_getfd(self.fd);
        if ret < 0 {
            Err(Errno(-ret as c_int))
        } else {
            Ok(ret & cty::O_CLOEXEC != 0)
        }
    }

    /// Enables or disables the `close on exec` flag.
    ///
    /// [argument, val]
    /// Whether the flag is set.
    ///
    /// = See also
    ///
    /// * link:man:fcntl(2) and the description of `F_SETFD` therein.
    /// * link:lrs::file::File::io_close_on_exec
    pub fn set_close_on_exec(&self, val: bool) -> Result {
        let mut ret = fcntl_getfd(self.fd);
        if ret >= 0 {
            ret = (ret & !cty::O_CLOEXEC) | (cty::O_CLOEXEC * val as c_int);
            ret = fcntl_setfd(self.fd, ret);
        }
        rv!(ret)
    }

    /// Reads from a position in the file.
    ///
    /// [argument, buf]
    /// The buffer that will be filled by the operation.
    ///
    /// [argument, off]
    /// The position in the file at which to read.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// This function does not change the read/write position of the file description.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:pread(2)
    /// * link:lrs::file::File::read
    /// * link:lrs::file::File::scatter_read
    /// * link:lrs::file::File::scatter_read_at
    pub fn read_at(&self, buf: &mut [u8], off: i64) -> Result<usize> {
        retry(|| pread(self.fd, buf, off as loff_t)).map(|r| r as usize)
    }

    /// Writes to an offset in the file.
    ///
    /// [argument, buf]
    /// The buffer that will be written to the file.
    ///
    /// [argument, off]
    /// The position in the file at which to write.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// This function does not change the read/write position of the file description.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:pwrite(2)
    /// * link:lrs::file::File::write
    /// * link:lrs::file::File::gather_write
    /// * link:lrs::file::File::gather_write_at
    pub fn write_at(&self, buf: &[u8], off: i64) -> Result<usize> {
        retry(|| pwrite(self.fd, buf, off as loff_t)).map(|r| r as usize)
    }

    /// Reads from the file into multiple buffers.
    ///
    /// [argument, bufs]
    /// The buffers that will be filled by the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// This operation is atomic in the sense that the read operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:readv(2)
    /// * link:lrs::file::File::read
    /// * link:lrs::file::File::read_at
    /// * link:lrs::file::File::scatter_read_at
    pub fn scatter_read(&self, bufs: &mut [&mut [u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| readv(self.fd, bufs)).map(|r| r as usize)
    }

    /// Writes from multiple buffers to the file.
    ///
    /// [argument, bufs]
    /// The buffers that will be written to the file.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// This operation is atomic in the sense that the write operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:writev(2)
    /// * link:lrs::file::File::write
    /// * link:lrs::file::File::write_at
    /// * link:lrs::file::File::gather_write_at
    pub fn gather_write(&self, bufs: &[&[u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| writev(self.fd, bufs)).map(|r| r as usize)
    }

    /// Reads from a position in the file into multiple buffers.
    ///
    /// [argument, bufs]
    /// The buffers that will be filled by the operation.
    ///
    /// [argument, off]
    /// The position in the file at which to read.
    ///
    /// [return_value]
    /// Returns the number of bytes read.
    ///
    /// = Remarks
    ///
    /// This function does not change the read/write position of the file description.
    ///
    /// This operation is atomic in the sense that the read operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If the length of the buffer is `0`, the meaning of a `0` return value is
    /// unspecified. Otherwise a return value of `0` signals End-Of-File.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:readv(2)
    /// * link:lrs::file::File::read
    /// * link:lrs::file::File::read_at
    /// * link:lrs::file::File::scatter_read
    pub fn scatter_read_at(&self, bufs: &mut [&mut [u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| preadv(self.fd, bufs, off as loff_t)).map(|r| r as usize)
    }

    /// Writes from multiple buffers to an offset in the file.
    ///
    /// [argument, bufs]
    /// The buffers that will be written to the file.
    ///
    /// [argument, off]
    /// The position in the file at which to write.
    ///
    /// [return_value]
    /// Returns the number of bytes written.
    ///
    /// = Remarks
    ///
    /// This function does not change the read/write position of the file description.
    ///
    /// This operation is atomic in the sense that the write operations will not be
    /// interleaved with other operations on the same file description.
    ///
    /// If lrs was compiled with the `retry` option, this call will automatically retry
    /// the operation if the call was interrupted by a signal.
    ///
    /// = See also
    ///
    /// * link:man:pwritev(2)
    /// * link:lrs::file::File::write
    /// * link:lrs::file::File::write_at
    /// * link:lrs::file::File::gather_write
    pub fn gather_write_at(&self, bufs: &[&[u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| pwritev(self.fd, bufs, off as loff_t)).map(|r| r as usize)
    }

    /// Changes the length of the file.
    ///
    /// [argument, len]
    /// The new length of the file.
    ///
    /// = Remarks
    ///
    /// :seek: link:lrs::file::File::seek
    ///
    /// If the length is larger than the current length, a hole is created. Such holes can
    /// be inspected with the {seek}[seek] method.
    ///
    /// = See also
    ///
    /// * link:man:ftruncate(2)
    /// * link:lrs::file::set_len
    /// * {seek}
    pub fn set_len(&self, len: i64) -> Result {
        retry(|| ftruncate(self.fd, len as loff_t)).map(|_| ())
    }

    /// Flushes all data and meta-data of the file to the disk.
    ///
    /// = See also
    ///
    /// * link:man:fsync(2)
    /// * link:lrs::file::File::data_sync
    pub fn sync(&self) -> Result {
        rv!(fsync(self.fd))
    }

    /// Flushes enough data and mate-data to the disk that the content of the file can be
    /// read again.
    ///
    /// = Remarks
    ///
    /// :sync: link:lrs::file::File::sync
    ///
    /// In some cases, this is more efficient than {sync}[sync].
    ///
    /// = See also
    ///
    /// * link:man:fdatasync(2)
    /// * {sync}
    pub fn data_sync(&self) -> Result {
        rv!(fdatasync(self.fd))
    }

    /// Flushes all data and meta-data of the filesystem containing the file to the disk.
    /// 
    /// = See also
    ///
    /// * link:man:syncfs(2)
    /// * link:lrs::file::File::sync
    pub fn sync_filesystem(&self) -> Result {
        rv!(syncfs(self.fd))
    }

    /// Advises the kernel that a range in the file will have a certain usage pattern.
    ///
    /// [argument, from]
    /// The start of the range.
    ///
    /// [argument, to]
    /// The end of the range.
    ///
    /// [argument, advice]
    /// The advice given to the kernel.
    ///
    /// = See also
    ///
    /// * link:man:fadvise(2)
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

    /// Returns information about the filesystem in which this file in stored.
    ///
    /// [return_value]
    /// Returns information about the filesystem.
    ///
    /// = See also
    /// 
    /// * link:man:fstatfs(2)
    pub fn fs_info(&self) -> Result<FileSystemInfo> {
        let mut buf = mem::zeroed();
        retry(|| fstatfs(self.fd, &mut buf)).map(|_| from_statfs(buf))
    }

    /// Creates a hard link to the file.
    ///
    /// [argument, path]
    /// The path at which the link will be created.
    ///
    /// = Remarks
    ///
    /// The new path must be in the same mount point as the opened file.
    ///
    /// In general, this function cannot be used if there are no links to the file, e.g.,
    /// because the last link was deleted after the file was opened. However, this is
    /// possible if the file was opened with the `temp file` and without the `exclusive`
    /// flag. This can be used to securely create files and only make them visible in the
    /// file system once they have been completely written.
    ///
    /// If the path is relative, it is interpreted relative to the current working
    /// directory.
    ///
    /// = See also
    ///
    /// * link:man:linkat(2)
    pub fn link<P>(&self, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(linkat(self.fd, CStr::empty(), AT_FDCWD, &path, AT_EMPTY_PATH))
    }

    /// Creates a hard link to this file relative to a directory.
    ///
    /// = Remarks
    ///
    /// Relative paths are interpreted relative to the directory `dir`.
    pub fn link_rel_to<P>(&self, dir: &File, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    /// = Remarks
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
    /// = Remarks
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
    /// = Remarks
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
        let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf));
        rv!(fsetxattr(self.fd, &name, val.as_ref(), 0))
    }

    /// Gets an attribute of this file.
    pub fn get_attr_buf<S, V>(&self, name: S, mut val: V) -> Result<usize>
        where S: ToCString, V: AsMut<[u8]>,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf));
        rv!(fgetxattr(self.fd, &name, val.as_mut()), -> usize)
    }

    /// Gets an attribute of this file.
    pub fn get_attr<S>(&self, name: S) -> Result<SVec<u8>>
        where S: ToCString,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf));
        get_attr_common(|buf| fgetxattr(self.fd, &name, buf))
    }

    /// Removes an attribute of this file.
    pub fn remove_attr<S>(&self, name: S) -> Result
        where S: ToCString,
    {
        let mut buf: [u8; 128] = unsafe { mem::uninit() };
        let name: Rmo<_, FbHeap> = try!(name.rmo_cstr(&mut buf));
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
    /// = Remarks
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
    /// = Remarks
    ///
    /// If `path` is relative, the `self` must be a directory and the `path` will be
    /// interpreted relative to `self`.
    pub fn rel_open<P>(&self, path: P, flags: Flags) -> Result<File>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    /// = Remarks
    ///
    /// If `path` is a symlink, then this is equivalent to returning information about the
    /// destination of the symlink. If `path` is relative, then `self` must be a directory
    /// and the path will be interpreted relative to `self`.
    pub fn rel_info<P>(&self, path: P) -> Result<Info>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let mut stat = mem::zeroed();
        try!(rv!(fstatat(self.fd, &path, &mut stat, 0)));
        Ok(info_from_stat(stat))
    }

    /// Returns information about the file specified by `path`.
    ///
    /// = Remarks
    ///
    /// This returns information about the file at `path`, even if `path` is a symlink.
    /// If `path` is relative, then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_info_no_follow<P>(&self, path: P) -> Result<Info>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let mut stat = mem::zeroed();
        try!(rv!(fstatat(self.fd, &path, &mut stat, AT_SYMLINK_NOFOLLOW)));
        Ok(info_from_stat(stat))
    }

    /// Returns whether the specified path points to an existing file.
    ///
    /// = Remarks
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_exists<P>(&self, path: P) -> Result<bool>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    /// = Remarks
    ///
    /// If `path` is relative then `self` must be a directory and the path will be
    /// interpreted relative to `self`.
    pub fn rel_can_access<P>(&self, path: P, mode: AccessMode) -> Result<bool>
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
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
    /// = Remarks
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the destination.
    pub fn rel_set_times<P>(&self, path: P, access: TimeChange,
                            modification: TimeChange) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, 0))
    }

    /// Changes the access and modification times of the file specified by `path`.
    ///
    /// = Remarks
    ///
    /// If `path` is relative then `self` has to be a directory and relative paths are
    /// interpreted relative to `self`. If `path` is a symlink, then this changes the
    /// times of the symlink.
    pub fn rel_set_times_no_follow<P>(&self, path: P, access: TimeChange,
                                      modification: TimeChange) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let times = [time_change_to_timespec(access),
                     time_change_to_timespec(modification)];
        rv!(utimensat(self.fd, Some(&path), &times, AT_SYMLINK_NOFOLLOW))
    }

    /// Atomically exchanges the two files `one` and `two`.
    ///
    /// = Remarks
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`.
    pub fn rel_exchange<P, Q>(&self, one: P, two: Q) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let one: Rmo<_, FbHeap> = try!(one.rmo_cstr(&mut buf1));
        let two: Rmo<_, FbHeap> = try!(two.rmo_cstr(&mut buf2));
        rv!(renameat(self.fd, &one, self.fd, &two, RENAME_EXCHANGE))
    }

    /// Renames `one` to `two`.
    ///
    /// = Remarks
    ///
    /// If one of the paths is relative, then `self` has to be a directory and the path
    /// will be interpreted relative to `self`. If `replace` is `false`, then the
    /// operation fails if `two` already exists.
    pub fn rel_rename<P, Q>(&self, one: P, two: Q, replace: bool) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let one: Rmo<_, FbHeap> = try!(one.rmo_cstr(&mut buf1));
        let two: Rmo<_, FbHeap> = try!(two.rmo_cstr(&mut buf2));
        let flag = if replace { 0 } else { RENAME_NOREPLACE };
        rv!(renameat(self.fd, &one, self.fd, &two, flag))
    }

    /// Creates the directory `path`.
    ///
    /// = Remarks
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`.
    pub fn rel_create_dir<P>(&self, path: P, mode: Mode) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(mkdirat(self.fd, &path, mode_to_int(mode)))
    }

    /// Removes the file at `path`.
    ///
    /// = Remarks
    ///
    /// If `path` is relative, then `self` has to be a directory and the path is
    /// interpreted relative to `self`. If `path` refers to a directory, then the
    /// directory has to be empty.
    pub fn rel_remove<P>(&self, path: P) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        let mut ret = unlinkat(self.fd, &path, 0);
        if Errno(-ret) == error::IsADirectory {
            ret = unlinkat(self.fd, &path, AT_REMOVEDIR);
        }
        rv!(ret)
    }

    /// Creates a symlink from `link` to `target`.
    ///
    /// = Remarks
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_symlink<P, Q>(&self, target: P, link: Q) -> Result
        where P: ToCString, Q: ToCString,
    {
        let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let target: Rmo<_, FbHeap> = try!(target.rmo_cstr(&mut buf1));
        let link: Rmo<_, FbHeap> = try!(link.rmo_cstr(&mut buf2));
        rv!(symlinkat(&target, self.fd, &link))
    }

    /// Reads the target of the symbolic link `link` into `buf`.
    ///
    /// = Remarks
    ///
    /// If `link` is relative, then `self` has to be a directory and `link` will be
    /// interpreted relative to `self`.
    pub fn rel_read_link_buf<'a, P>(&self, link: P,
                                    buf: &'a mut [u8]) -> Result<&'a mut NoNullStr>
        where P: ToCString,
    {
        let mut pbuf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let link: Rmo<_, FbHeap> = try!(link.rmo_cstr(&mut pbuf));
        let len = try!(rv!(readlinkat(self.fd, &link, buf), -> usize));
        Ok(unsafe { NoNullStr::from_bytes_unchecked_mut(&mut buf[..len]) })
    }

    /// Reads the target of the symbolic link `link`.
    ///
    /// = Remarks
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
    /// = Remarks
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_owner<P>(&self, path: P, user: UserId, group: GroupId) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, 0))
    }

    /// Changes the owner of the file at `path`.
    ///
    /// = Remarks
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`. If `path` refers to a symlink, then this changes
    /// the owner of the symlink itself.
    pub fn rel_change_owner_no_follow<P>(&self, path: P, user: UserId,
                                         group: GroupId) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(fchownat(self.fd, &path, user, group, AT_SYMLINK_NOFOLLOW))
    }

    /// Change the mode of the file at `path`.
    ///
    /// = Remarks
    ///
    /// If `path` is relative, then `self` has to be a directory and `path` will be
    /// interpreted relative to `self`.
    pub fn rel_change_mode<P>(&self, path: P, mode: Mode) -> Result
        where P: ToCString,
    {
        let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(fchmodat(self.fd, &path, mode_to_int(mode)))
    }

    /// Creates a file at `path`.
    ///
    /// = Remarks
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
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), 0))
    }

    /// Creates a device special file at `path`.
    ///
    /// = Remarks
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
        let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
        rv!(mknodat(self.fd, &path, file_type_to_mode(ty) | mode_to_int(mode), dev.id()))
    }
}

impl Read for File {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        File::scatter_read(self, buf)
    }
}

impl Write for File {
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize> {
        File::gather_write(self, buf)
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
