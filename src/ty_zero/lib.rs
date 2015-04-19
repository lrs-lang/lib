// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_ty_zero"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_cty_base as cty_base;

pub mod linux {
    pub use core::linux::*;
    pub use {clone, result};
}

pub mod clone;
pub mod result;
pub mod error;

pub mod prelude {
    pub use core::prelude::*;
    pub use result::{Result};
    pub use result::Result::{Ok, Err};
}
