// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux"]
#![crate_type = "lib"]
#![feature(std_misc, core, io, plugin, asm)]
#![plugin(rest_easy)]

pub mod file;
pub mod user;
pub mod group;
pub mod errno;
pub mod result;
pub mod fs;
pub mod process;

mod imp;
