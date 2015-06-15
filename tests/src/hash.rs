// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std, asm)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
extern crate lrs_hash as hash;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::fd::{STDIN};
use lrs::process::{process_id, exit};

fn main() {
    static INPUT: [u8; 100_000_000] = [0; 100_000_000];
    let mut i = 0;
    while i < INPUT.len() {
        unsafe { asm!("" : : "r"(INPUT[i])); }
        i += 4096;
    }
    println!("{}", process_id());
    STDIN.read(&mut [0]);
    let hash = hash::xx_hash::u32hash_bytes(&INPUT, 0);
    exit(hash as u8);
}
