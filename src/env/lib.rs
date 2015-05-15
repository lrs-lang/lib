// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_env"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core      as core;
extern crate lrs_base      as base;
extern crate lrs_str_one   as str_one;
extern crate lrs_str_two   as str_two;
extern crate lrs_str_three as str_three;
extern crate lrs_rt        as rt;
extern crate lrs_cty       as cty;
extern crate lrs_rmo       as rmo;
extern crate lrs_alloc     as alloc;
extern crate lrs_syscall   as syscall;

#[prelude_import] use base::prelude::*;
use core::slice::{Split};
use core::{mem};
use str_one::{AsByteStr, CStr, NoNullStr};
use str_two::{NoNullString};
use str_three::{ToCString};
use alloc::{Allocator, FbHeap};
use rt::{env};
use base::{error};
use cty::{PATH_MAX};
use rmo::{Rmo};

mod lrs { pub use base::lrs::*; pub use cty; }

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
pub fn var<S>(name: S) -> Result<&'static CStr>
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
        self.path.next().map(|p| unsafe { NoNullStr::from_bytes_unchecked(p) })
    }
}

/// Retrieves the current working directory.
///
/// [argument, buf]
/// The buffer in which the current working directory will be stored.
pub fn cwd<H>(buf: &mut NoNullString<H>) -> Result
    where H: Allocator,
{
    for &res in &[0, 128, 256, 512, 1024, 2048, 4096][..] {
        try!(buf.reserve(res));
        let size = match rv!(syscall::getcwd(buf.unused()), -> usize) {
            Ok(s) => s,
            Err(error::RangeError) => continue,
            Err(e) => return Err(e),
        };
        let buf_len = buf.len();
        unsafe { buf.set_len(buf_len + size - 1); }
        return Ok(());
    }
    // This should never happen because the kernel returns PathTooLong if the cwd doesn't
    // fit in one page = 4096 bytes which is the last thing we try above.
    //
    // XXX: However, some platforms use a larger page size.
    abort!();
}

/// Sets the current working directory.
///
/// [argument, path]
/// The path of the new current working directory.
pub fn set_cwd<P>(path: P) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    rv!(syscall::chdir(&path))
}
