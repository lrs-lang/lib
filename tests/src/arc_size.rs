// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
extern crate lrs_base;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs_base::unused::{UnusedState};
use lrs::{mem, rc};

fn main() {
    rc::Arc::<u8>::unused_state(0);
    println!("{}", mem::size_of::<rc::Arc<u8>>());
}

