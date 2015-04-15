// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_stdio"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_ty_one as ty_one;
extern crate linux_io as io;

use core::prelude::*;
use ty_one::error::{UNKNOWN};

pub struct Stdout;

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        extern {
            fn write(fd: i32, buf: *const u8, len: u64) -> i64;
        }
        let res = unsafe { write(1, buf.as_ptr(), buf.len() as u64) };
        if res < 0 {
            Err(UNKNOWN)
        } else {
            Ok(res as usize)
        }
    }
}
