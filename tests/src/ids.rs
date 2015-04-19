// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::{process};

fn main() {
    let user_ids = process::UserIds::get();
    println!("{:?}", user_ids);

    let mut sups = [0; 20];
    let _ = process::supplementary_groups(&mut sups);
    println!("{:?}", &sups[..]);
}
