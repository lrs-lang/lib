// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_ty_one"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;

pub mod linux {
    pub use ::core::linux::*;
    pub use {result};
}

pub mod error;
pub mod c_str;
pub mod parse;
pub mod cow;
pub mod byte_str;
pub mod saturating_cast;
pub mod as_bytes;
pub mod result;

pub mod prelude {
    pub use result::{Result};
    pub use result::Result::{Ok, Err};
}
