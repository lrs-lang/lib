// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;

use linux::{file, time};

fn main() {
    let info = file::info_no_follow("/etc/localtime").unwrap();
    let last_mod = info.last_modification();
    let tokyo = time::Zone::load("Asia/Tokyo").unwrap();
    let expanded = tokyo.expand(last_mod);

    println!("{:?}", expanded);
}
