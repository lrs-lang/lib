// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::env::{cwd};
use linux::string::{NoNullString};

fn main() {
    let mut path_buf = [0; 32];
    let mut path = NoNullString::buffered(&mut path_buf);
    println!("{:?}", cwd(&mut path));

    println!("{:?}", path);
}
