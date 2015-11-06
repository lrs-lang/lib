// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, asm)]
#![plugin(lrs_core_plugin)]

extern crate lrs_hash as hash;

use std::fd::{STDIN};
use std::process::{process_id, exit};

fn main() {
    // LLVM places immutable statics in .rodata which blows the binary up to 100MB if we
    // don't use this attribute.
    #[link_section = ".bss"]
    static INPUT: [u8; 100_000_000] = [0; 100_000_000];

    // Touch every page in INPUT so that they are not swapped out when we measure the
    // speed below.
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
