// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::time::{self, Time};
use lrs::tty::{Tty};
use lrs::{process};
use lrs::file::flags::{FILE_READ_WRITE};

fn main() {
    println!("{}", process::process_id());
    let tty = Tty::from_borrowed(0);
    println!("{:?}", tty.attributes());
}
