// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_base"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_attribute, custom_derive, prelude_import)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_wrapping as wrapping;
extern crate lrs_cty_base as cty_base;

pub mod std {
    pub use core::*;
    pub use {result, clone, error};
}

pub mod clone;
pub mod result;
pub mod error;
pub mod rmo;
pub mod into;
pub mod undef;
pub mod default;

pub mod prelude {
    pub use core::prelude::v1::*;
    pub use core::bool::{BoolExt};
    pub use result::{Result};
    pub use result::Result::{Ok, Err};
    pub use rmo::{AsRef, AsMut};
    pub use into::{Into};
}
