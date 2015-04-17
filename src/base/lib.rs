// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_base"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
// #![allow(trivial_numeric_casts, trivial_casts)]

#[macro_use]
extern crate linux_core as core;
extern crate linux_ty_one as ty_one;
extern crate linux_arch as arch;

#[macro_use]
pub mod macros;
pub mod util;
pub mod alias;
pub mod fd_container;
