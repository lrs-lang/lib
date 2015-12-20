// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_file"]
#![crate_type = "lib"]
#![feature(no_std, custom_derive)]
#![no_std]

extern crate lrs_base      as base;
extern crate lrs_io        as io;
extern crate lrs_cty       as cty;
extern crate lrs_int       as int;
extern crate lrs_syscall   as syscall;
extern crate lrs_str_one   as str_one;
extern crate lrs_str_two   as str_two;
extern crate lrs_arch_fns  as arch_fns;
extern crate lrs_rv        as rv;
extern crate lrs_fmt       as fmt;
extern crate lrs_vec       as vec;
extern crate lrs_rmo       as rmo;
extern crate lrs_parse     as parse;
extern crate lrs_fd        as fd;
extern crate lrs_alloc     as alloc;
extern crate lrs_dev       as dev;
extern crate lrs_cell      as cell;
extern crate lrs_fs        as fs;
extern crate lrs_time_base as time_base;

mod std { pub use vec::std::*; pub use {cty}; }

use base::prelude::*;
use vec::{Vec};
use core::{mem};
use base::error::{self};
use cty::{loff_t, AT_FDCWD, UTIME_NOW, UTIME_OMIT, timespec, PATH_MAX, ssize_t};
use syscall::{
    truncate, linkat, setxattr, lsetxattr, getxattr, lgetxattr, removexattr, lremovexattr,
    listxattr, llistxattr,
};
use str_one::{CStr};
use str_two::{CString};
use arch_fns::{memchr};
use rv::{retry};
use cty::alias::{UserId, GroupId};
use rmo::{Rmo, ToRmo};
use alloc::{FbHeap, FcPool, OncePool, MemPool};
use time_base::{Time, time_to_timespec};
use dev::{Device};
use cell::{Cell};

use flags::{Mode, AccessMode};
use info::{Info, Type};

pub use file::{File, Advice, Seek};

pub mod flags;
pub mod info;
mod file;

type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [d8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let old = try!(rmo_cstr(&source, &mut buf1));
    let new = try!(rmo_cstr(&link, &mut buf2));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
/// If the paths are relative, they will be interpreted relative to the current working
/// directory. Both paths must refer to the same mount point or the operation fails.
///
/// == Kernel versions
///
/// The required kernel version is 3.15.
///
/// = See also
///
/// * link:man:renameat2(2)
/// * link:lrs::file::File::rel_exchange
pub fn exchange<P, Q>(one: P, two: Q) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
/// directory. Both paths must refer to the same mount point or the operation fails.
///
/// == Kernel versions
///
/// If `replace` is `false`, the required kernel version is 3.15.
///
/// = See also
///
/// * link:man:renameat2(2)
/// * link:lrs::file::File::rel_rename
pub fn rename<P, Q>(from: P, to: Q, replace: bool) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
/// * link:lrs::file::File::rel_read_link_pool
pub fn read_link_pool<L, P = alloc::Heap>(link: L, pool: P) -> Result<CString<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool,
{
    File::current_dir().rel_read_link_pool(link, pool)
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
/// * link:lrs::file::read_link_pool
/// * link:lrs::file::File::rel_read_link
pub fn read_link<L, P = alloc::Heap>(link: L) -> Result<CString<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool+OutOf,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
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
pub fn set_attr<P, S, V: ?Sized>(path: P, name: S, val: &V) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          V: AsRef<[d8]>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
pub fn set_attr_no_follow<P, S, V: ?Sized>(path: P, name: S, val: &V) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          V: AsRef<[d8]>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
pub fn get_attr_buf<P, S>(path: P, name: S, buf: &mut [d8]) -> Result<usize>
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
pub fn get_attr_no_follow_buf<P, S, V>(path: P, name: S, buf: &mut [d8]) -> Result<usize>
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
    rv!(lgetxattr(&path, &name, buf), -> usize)
}

fn get_attr_common<F>(mut f: F) -> Result<Vec<u8>>
    where F: FnMut(&mut [d8]) -> ssize_t,
{
    let mut vec = vec!();
    loop {
        let size = try!(rv!(f(&mut []), -> usize));
        vec.reserve(size);
        match rv!(f(vec.unused()), -> usize) {
            Ok(n) => {
                unsafe { vec.set_len(n); }
                return Ok(vec);
            },
            Err(error::RangeError) => { },
            Err(e) => return Err(e),
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
pub fn get_attr<P, S>(path: P, name: S) -> Result<Vec<u8>>
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
pub fn get_attr_no_follow<P, S>(path: P, name: S) -> Result<Vec<u8>>
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; 128] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf1));
    let name = try!(rmo_cstr(&name, &mut buf2));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
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
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(llistxattr(&path, &mut []), -> usize)
}

fn list_attr_common<F, P>(pool: P, mut f: F) -> Result<ListAttrIter<P>>
    where F: FnMut(&mut [d8]) -> ssize_t,
          P: MemPool,
{
    let mut vec = Vec::with_pool(pool);
    loop {
        let size = try!(rv!(f(&mut []), -> usize));
        vec.reserve(size);
        match rv!(f(vec.unused()), -> usize) {
            Ok(n) => {
                unsafe { vec.set_len(n); }
                return Ok(ListAttrIter { buf: vec, pos: Cell::new(0) });
            },
            Err(error::RangeError) => { },
            Err(e) => return Err(e),
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
pub fn list_attr_pool<L, P>(path: L, pool: P) -> Result<ListAttrIter<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    list_attr_common(pool, |buf| listxattr(&path, buf))
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
pub fn list_attr<L, P = alloc::Heap>(path: L) -> Result<ListAttrIter<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool+OutOf,
{
    list_attr_pool(path, P::out_of(()))
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
pub fn list_attr_no_follow_pool<L, P>(path: L, pool: P) -> Result<ListAttrIter<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    list_attr_common(pool, |buf| llistxattr(&path, buf))
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
pub fn list_attr_no_follow<L, P = alloc::Heap>(path: L) -> Result<ListAttrIter<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool+OutOf,
{
    list_attr_no_follow_pool(path, P::out_of(()))
}

/// Returns a canonicalized absolute path.
///
/// [argument, path]
/// The path to canonicalize.
///
/// [argument, buf]
/// The buffer where the path will be stored in.
///
/// = Remarks
///
/// The path will not contain any `/./`, `/../`, or `//`.
///
/// = See also
///
/// * link:man:realpath(3)
pub fn real_path_pool<L, P>(path: L, pool: P) -> Result<CString<P>>
    where L: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
          P: MemPool,
{
    File::current_dir().rel_real_path_pool(path, pool)
}

/// Returns a canonicalized absolute path.
///
/// [argument, path]
/// The path to canonicalize.
///
/// = Remarks
///
/// The path will not contain any `/./`, `/../`, or `//`.
///
/// = See also
///
/// * link:man:realpath(3)
pub fn real_path<L, P = alloc::Heap>(path: L) -> Result<CString<P>>
    where L: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          P: MemPool+OutOf,
{
    File::current_dir().rel_real_path(path)
}

/// A time change.
pub enum TimeChange {
    /// Does not modify the time.
    Omit,
    /// Sets the time to the current time.
    Now,
    /// Sets the time to the specified time.
    ///
    /// [field, 1]
    /// The time to be set.
    Set(Time),
}

fn time_change_to_timespec(t: TimeChange) -> timespec {
    match t {
        TimeChange::Omit => timespec { tv_sec: 0, tv_nsec: UTIME_OMIT },
        TimeChange::Now  => timespec { tv_sec: 0, tv_nsec: UTIME_NOW  },
        TimeChange::Set(v) => time_to_timespec(v),
    }
}

/// An iterator over file attributes.
pub struct ListAttrIter<P>
    where P: MemPool,
{
    buf: Vec<u8, P>,
    pos: Cell<usize>,
}

impl<P> ListAttrIter<P>
    where P: MemPool,
{
    pub fn next(&self) -> Option<&CStr> {
        let buf = &self.buf[self.pos.get()..];
        let len = match memchr(buf, 0) {
            Some(l) => l,
            _ => return None,
        };
        self.pos.set(self.pos.get() + len + 1);
        Some(unsafe { mem::cast(&buf[..len]) })
    }
}
