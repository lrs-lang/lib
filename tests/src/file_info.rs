// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;

use lrs::file::{File};

fn main() {
    let file = File::open_read("/usr/bin/sudo").unwrap();
    println!("{:?}", file.info());
}
