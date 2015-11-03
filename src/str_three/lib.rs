// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_str_three"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_arch_fns as arch_fns;
extern crate lrs_base     as base;
extern crate lrs_rmo      as rmo;
extern crate lrs_str_one  as str_one;
extern crate lrs_str_two  as str_two;
extern crate lrs_alloc    as alloc;

pub use c_string::{ToCString};

mod std { pub use base::std::*; }

pub mod c_string;
