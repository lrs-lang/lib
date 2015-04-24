// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_str_two"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core     as core;
extern crate linux_arch_fns as arch_fns;
extern crate linux_base     as base;
extern crate linux_str_one  as str_one;
extern crate linux_alloc    as alloc;
extern crate linux_fmt      as fmt;
extern crate linux_vec      as vec;

pub use byte_string::{ByteString};
pub use no_null_string::{NoNullString};
pub use c_string::{CString};
pub use string::{String};

mod linux { pub use fmt::linux::*; }

pub mod byte_string;
pub mod c_string;
pub mod string;
pub mod no_null_string;
