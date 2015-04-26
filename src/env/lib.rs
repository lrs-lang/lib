// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_env"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core      as core;
extern crate linux_base      as base;
extern crate linux_str_one   as str_one;
extern crate linux_str_two   as str_two;
extern crate linux_str_three as str_three;
extern crate linux_rt        as rt;
extern crate linux_cty       as cty;
extern crate linux_rmo       as rmo;
extern crate linux_alloc     as alloc;
extern crate linux_syscall   as syscall;

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

mod linux { pub use base::linux::*; pub use cty; }

pub fn var<S>(name: S) -> Result<&'static CStr>
    where S: AsByteStr,
{
    let bytes = name.as_byte_str();
    for var in env() {
        if var == bytes {
            return Ok(CStr::empty());
        }
        if let Some(var_name) = var.as_ref().split(|&b| b == b'=').next() {
            if var_name == bytes {
                let len = var_name.len() + 1;
                return Ok(&var[len..]);
            }
        }
    }
    Err(error::DoesNotExist)
}

fn path_split(b: &u8) -> bool { *b == b':' }

pub fn path() -> Result<PathIter> {
    Ok(PathIter { path: try!(var("PATH")).as_ref().split(path_split) })
}

pub struct PathIter {
    path: Split<'static, u8, fn(&u8) -> bool>,
}

impl Iterator for PathIter {
    type Item = &'static NoNullStr;
    fn next(&mut self) -> Option<&'static NoNullStr> {
        self.path.next().map(|p| unsafe { NoNullStr::from_bytes_unchecked(p) })
    }
}

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
    abort!();
}

pub fn set_cwd<P>(path: P) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let path: Rmo<_, FbHeap> = try!(path.rmo_cstr(&mut buf));
    rv!(syscall::chdir(&path))
}
