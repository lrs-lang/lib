// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_env"]
#![crate_type = "lib"]
#![no_std]

extern crate lrs_base    as base;
extern crate lrs_str_one as str_one;
extern crate lrs_str_two as str_two;
extern crate lrs_rt      as rt;
extern crate lrs_cty     as cty;
extern crate lrs_rmo     as rmo;
extern crate lrs_alloc   as alloc;
extern crate lrs_vec     as vec;
extern crate lrs_syscall as syscall;

use base::prelude::*;
use core::slice::{Split};
use core::{mem};
use str_one::{CStr, NoNullStr};
use str_two::{CString};
use alloc::{MemPool, FbHeap, FcPool, OncePool};
use rt::{env};
use vec::{Vec};
use base::{error};
use cty::{PATH_MAX};
use rmo::{Rmo, ToRmo};

mod std { pub use base::std::*; pub use cty; }

/// Retrieves the value of an environment variable, if any.
///
/// [argument, name]
/// The name of the variable to be found.
///
/// [return_value]
/// Returns the value of the variable with name `name` or an empty string if the variable
/// does not have a value.
///
/// = Remarks
///
/// This function iterates over all environment variables. If if finds a variable whose
/// whole string is equal to `name`, the empty string is returned. Otherwise it tries to
/// split the variable at the first `=` character and compares the part before the `=` to
/// `name`. If they compare equal, the part after the `=` is returned.
pub fn var<S: ?Sized>(name: &S) -> Result<&'static CStr>
    where S: AsRef<[u8]>
{
    let bytes = name.as_ref();
    for var in env() {
        if var == bytes {
            return Ok(CStr::empty());
        }
        let var_bytes: &[u8] = var.as_ref();
        if let Some(var_name) = var_bytes.split(|&b| b == b'=').next() {
            if var_name == bytes {
                let len = var_name.len() + 1;
                return Ok(&var[len..]);
            }
        }
    }
    Err(error::DoesNotExist)
}

fn path_split(b: &u8) -> bool { *b == b':' }

/// Returns an iterator over the paths in the `PATH` environment variable.
pub fn path() -> Result<PathIter> {
    let bytes: &[u8] = try!(var("PATH")).as_ref();
    Ok(PathIter { path: bytes.split(path_split) })
}

/// An iterator over the paths in the `PATH` environment variable.
pub struct PathIter {
    path: Split<'static, u8, fn(&u8) -> bool>,
}

impl Iterator for PathIter {
    type Item = &'static NoNullStr;
    fn next(&mut self) -> Option<&'static NoNullStr> {
        self.path.next().map(|p| unsafe { mem::cast(p) })
    }
}

/// Retrieves the current working directory.
///
/// [argument, buf]
/// The buffer in which the current working directory will be stored.
pub fn get_cwd<H = alloc::Heap>() -> Result<CString<H>>
    where H: MemPool+OutOf,
{
    get_cwd_pool(H::out_of(()))
}

/// Retrieves the current working directory.
///
/// [argument, buf]
/// The buffer in which the current working directory will be stored.
pub fn get_cwd_pool<H>(pool: H) -> Result<CString<H>>
    where H: MemPool,
{
    let mut buf = Vec::with_pool(pool);
    for &res in &[32, 128, 256, 512, 1024, 2048, rt::aux::page_size()][..] {
        try!(buf.reserve(res));
        let size = match rv!(syscall::getcwd(buf.unused()), -> usize) {
            Ok(s) => s,
            Err(error::RangeError) => continue,
            Err(e) => return Err(e),
        };
        unsafe {
            buf.set_len(size);
            return Ok(CString::from_bytes_unchecked(buf));
        }
    }
    // This should never happen because the kernel returns PathTooLong if the cwd doesn't
    // fit in one page which is the last thing we try above.
    //
    // XXX: However, some platforms use a dynamic page size.
    abort!();
}

pub type Pool<'a> = FcPool<OncePool<'a>, FbHeap>;

fn rmo_cstr<'a, S>(s: &'a S,
                   buf: &'a mut [d8]) -> Result<Rmo<'a, CStr, CString<Pool<'a>>>>
    where S: for<'b> ToRmo<Pool<'b>, CStr, CString<Pool<'b>>>,
{
    s.to_rmo_with(FcPool::new(OncePool::new(buf), FbHeap::out_of(())))
}

/// Sets the current working directory.
///
/// [argument, path]
/// The path of the new current working directory.
pub fn set_cwd<P>(path: P) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let path = try!(rmo_cstr(&path, &mut buf));
    rv!(syscall::chdir(&path))
}
