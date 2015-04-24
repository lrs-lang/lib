// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_env"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_str_one as str_one;
extern crate linux_rt as rt;

#[prelude_import] use base::prelude::*;
use core::slice::{Split};
use str_one::{AsByteStr, CStr, NoNullStr};
use rt::{env};
use base::{error};

mod linux { pub use base::linux::*; }

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
