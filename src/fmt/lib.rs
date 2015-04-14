// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_fmt"]
// #![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_error as error;
extern crate linux_io as io;

#[prelude_import] use core::prelude::*;
use io::{Write};

pub use num::{format_u64};

mod num;
mod str;

pub type Result = core::prelude::Result<(), error::Errno>;

trait Debug {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result;
}

trait Display {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result;
}

fn main() {
    extern {
        fn write(fd: i32, ptr: *const u8, len: u64);
    }
    let mut buf = [0; 200];
    Display::fmt("hello wö\\r\"ld\n", &mut &mut buf[..]);
    unsafe { write(1, buf.as_ptr(), 200); }
}
