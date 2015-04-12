// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_core"]
#![crate_type = "lib"]
#![feature(std_misc, io, into_cow, core)]
#![allow(trivial_numeric_casts, trivial_casts)]

extern crate linux_arch as arch;

pub use arch::{cty};

// XXX: Maybe move some of these out? core takes a long time to compile right now.

#[macro_use]
pub mod macros;
pub mod syscall;
pub mod ext;
pub mod string;
pub mod c_str;
pub mod result;
pub mod errno;
pub mod util;
pub mod alias;
pub mod fd_container;
