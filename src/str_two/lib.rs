// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_str_two"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core     as core;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_base     as base;
extern crate lrs_str_one  as str_one;
extern crate lrs_alloc    as alloc;
extern crate lrs_fmt      as fmt;
extern crate lrs_vec      as vec;

pub use byte_string::{ByteString};
pub use no_null_string::{NoNullString};
pub use c_string::{CString};
pub use string::{String};

pub type SByteString   <H = alloc::Heap> = ByteString   <'static, H>;
pub type SNoNullString <H = alloc::Heap> = NoNullString <'static, H>;
pub type SCString      <H = alloc::Heap> = CString      <'static, H>;
pub type SString       <H = alloc::Heap> = String       <'static, H>;

mod lrs { pub use fmt::lrs::*; }

pub mod byte_string;
pub mod c_string;
pub mod string;
pub mod no_null_string;
mod cmp;
