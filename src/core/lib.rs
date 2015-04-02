// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_core"]
#![crate_type = "lib"]
#![feature(std_misc, plugin, asm, io, into_cow, convert)]
#![allow(trivial_numeric_casts, trivial_casts)]

extern crate linux_arch;

pub use linux_arch::{arch, cty};

// XXX: Maybe move some of these out? core takes a long time to compile right now.

pub mod syscall;
pub mod ext;
pub mod string;
pub mod result;
pub mod errno;
pub mod util;
pub mod alias;
