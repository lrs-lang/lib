// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_str_one"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_cty_base as cty_base;
extern crate linux_base as base;
extern crate linux_arch_fns as arch_fns;
extern crate linux_parse as parse;
extern crate linux_fmt as fmt;

pub use byte_str::{ByteStr, AsByteStr, AsMutByteStr};
pub use no_null_str::{NoNullStr, AsNoNullStr, AsMutNoNullStr};
pub use c_str::{CStr, AsCStr, AsMutCStr, ToCStr};

pub mod linux {
    pub use ::base::linux::*;
}

pub mod byte_str;
pub mod no_null_str;
pub mod c_str;
