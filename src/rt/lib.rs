// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_rt"]
#![crate_type = "lib"]
#![feature(plugin, no_std, lang_items)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_cty_base as cty_base;
extern crate linux_str_one as str_one;
extern crate linux_libc as libc;

#[prelude_import] use base::prelude::*;
use core::{mem};
use core::iter::{Iterator};
use str_one::{CStr};
use cty_base::types::{c_char};

mod linux { pub use base::linux::*; }

static mut ARGC: isize = 0;
static mut ARGV: *const *const u8 = 0 as *const *const u8;

#[lang = "start"]
fn lang_start(main: *const u8, argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        ARGC = argc;
        ARGV = argv;
        mem::cast::<_, fn()>(main)();
    }
    0
}

pub fn arg_count() -> usize {
    unsafe { ARGC as usize }
}

pub fn args() -> ArgsIter {
    unsafe { ArgsIter { argv: ARGV } }
}

pub struct ArgsIter {
    argv: *const *const u8,
}

impl Iterator for ArgsIter {
    type Item = &'static CStr;
    fn next(&mut self) -> Option<&'static CStr> {
        unsafe {
            let arg = *self.argv;
            if arg.is_null() {
                None
            } else {
                self.argv = self.argv.add(1);
                Some(CStr::from_ptr(arg as *const c_char))
            }
        }
    }
}

pub fn env() -> EnvIter {
    unsafe { ArgsIter { argv: libc::environ } }
}

pub type EnvIter = ArgsIter;
