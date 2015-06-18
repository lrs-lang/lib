// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hashmap"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_alloc as alloc;
extern crate lrs_hash as hash;
extern crate lrs_fmt as fmt;

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::unused::{UnusedState};
use alloc::{Allocator};
use hash::{Hash, Hasher};
use hash::xx_hash::{XxHash32};

pub mod lrs { pub use fmt::lrs::*; }

mod bucket;
mod table;
