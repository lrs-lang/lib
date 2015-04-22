// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[allow(unused_imports)] #[prelude_import] use linux::prelude::*;

use linux::env::{args, arg_count, env};

fn main() {
    println!("Have {} args:", arg_count());
    for arg in args() {
        println!("{:?}", arg);
    }
    println!("");
    println!("environment:");
    for arg in env() {
        println!("{:?}", arg);
    }
}
