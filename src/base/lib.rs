// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_base"]
#![crate_type = "lib"]
#![feature(plugin, no_std, macro_reexport)]
#![plugin(linux_core_plugin)]
#![no_std]
// #![allow(trivial_numeric_casts, trivial_casts)]

#[macro_use]
#[macro_reexport(abort, assert, try, println, matches, vec, format)]
extern crate linux_core;
extern crate linux_sort;
extern crate linux_io;
extern crate linux_fmt;
extern crate linux_ty_one;
extern crate linux_lock;
extern crate linux_ty_two;
extern crate linux_arch_fns;
extern crate linux_arch;

pub use linux_core::{intrinsics, marker, mem, ops, option, ptr, repr, slice, str,
                     char};
pub use linux_ty_one::{clone, error, result, parse, bytes, range};
pub use linux_ty_two::{vec, rmo};
pub use linux_arch::{cty, syscall, atomic};
pub use linux_sort as sort;
pub use linux_fmt as fmt;

#[macro_use]
pub mod macros;
pub mod util;
pub mod alias;
pub mod fd_container;
pub mod raw_stdio;

pub mod num;
pub mod string;
pub mod cell;
pub mod cmp;
pub mod sync;
pub mod io;
pub mod iter;
pub mod path;

pub mod linux {
    pub use {fmt, clone, result, marker, ops, intrinsics, mem, vec};
}

pub mod core {
    pub use {intrinsics, option, iter};
}

pub mod prelude {
    pub use linux_ty_two::prelude::*;
    pub use iter::*;
}
