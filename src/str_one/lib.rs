// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_str_one"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_cty_base as cty_base;
extern crate lrs_base as base;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_parse as parse;
extern crate lrs_fmt as fmt;

pub use byte_str::{ByteStr, AsByteStr, AsMutByteStr};
pub use no_null_str::{NoNullStr, AsNoNullStr, AsMutNoNullStr};
pub use c_str::{CStr, AsCStr, AsMutCStr, ToCStr};

pub mod lrs {
    pub use ::base::lrs::*;
}

pub mod byte_str;
pub mod no_null_str;
pub mod c_str;
