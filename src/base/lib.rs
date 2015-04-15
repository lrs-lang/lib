// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_base"]
#![crate_type = "lib"]
#![feature(std_misc, into_cow, core, plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts, trivial_casts)]

extern crate linux_core as core;
extern crate linux_error as error;
extern crate linux_cty;
extern crate linux_syscall as raw_syscall;

pub use linux_cty as cty;

// XXX: Maybe move some of these out? core takes a long time to compile right now.

#[macro_use]
pub mod macros;
// pub mod syscall;
// pub mod ext;
// pub mod string;
// pub mod c_str;
pub mod result;
// pub mod errno;
pub mod util;
// pub mod alias;
// pub mod fd_container;
