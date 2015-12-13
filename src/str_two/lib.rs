// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_str_two"]
#![crate_type = "lib"]
#![feature(no_std)]
#![no_std]

extern crate lrs_arch_fns as arch_fns;
extern crate lrs_base     as base;
extern crate lrs_str_one  as str_one;
extern crate lrs_alloc    as alloc;
extern crate lrs_fmt      as fmt;
extern crate lrs_vec      as vec;
extern crate lrs_box      as bx;

pub use c_string::{CString};
pub use string::{String};

mod std { pub use fmt::std::*; }

pub mod c_string;
pub mod string;
mod cmp;
mod conv;
