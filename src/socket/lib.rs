// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_socket"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core     as core;
extern crate lrs_base     as base;
extern crate lrs_cty      as cty;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_str_one  as str_one;
extern crate lrs_fmt      as fmt;

pub use addr::{SockAddr, AddrType};
pub use addr::unix::{UnixSockAddr, UnixAddrType};

mod lrs { pub use fmt::lrs::*; }

mod addr;
