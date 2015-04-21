// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_str_three"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core     as core;
extern crate linux_arch_fns as arch_fns;
extern crate linux_base  as base;
extern crate linux_rmo      as rmo;
extern crate linux_str_one  as str_one;
extern crate linux_str_two  as str_two;

pub use c_string::{ToCString};

mod linux { pub use base::linux::*; }

pub mod c_string;
