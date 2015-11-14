// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_rt"]
#![crate_type = "lib"]
#![feature(plugin, no_std, lang_items, link_args, asm)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty_base as cty_base;
extern crate lrs_cty as cty;
extern crate lrs_str_one as str_one;
extern crate lrs_libc as libc;
extern crate lrs_syscall as syscall;
extern crate lrs_r_syscall as r_syscall;

use base::prelude::*;
use core::{mem};
use str_one::{CStr};
use cty_base::types::{c_char};

mod std { pub use base::std::*; pub use cty; }

#[cfg(no_libc)] #[path = "no_libc/mod.rs"] pub mod imp;
#[cfg(not(no_libc))] #[path = "libc/mod.rs"]  pub mod imp;

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

/// Returns the number of command line arguments.
pub fn arg_count() -> usize {
    unsafe { ARGC as usize }
}

/// Returns an iterator over the command line arguments.
pub fn args() -> ArgsIter {
    unsafe { ArgsIter { argv: ARGV } }
}

/// An iterator over the command line arguments.
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

pub fn raw_env() -> *const *const c_char {
    imp::raw_env()
}

/// Returns an iterator over the environment variables.
pub fn env() -> EnvIter {
    ArgsIter { argv: raw_env() as *const _ }
}

pub type EnvIter = ArgsIter;
