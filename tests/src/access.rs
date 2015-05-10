// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::parse::{Parse};
use lrs::file::{can_access};

fn main() {
    assert!(can_access("Makefile", "rw-".parse().unwrap()) == Ok(true));
    assert!(can_access("Makefile", "--x".parse().unwrap()) == Ok(false));
}
