// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::alloc::{TaAlloc, TaPool};

fn main() {
    let mut libc_vec: Vec<u8> = Vec::new();
    let mut jemalloc_vec: Vec<u8, Jemalloc> = Vec::new();

    let mut buf = &mut [0; 4096][..];
    let mut stack_vec: Vec<usize, TaAlloc> = Vec::with_pool(TaPool::new(&mut buf));
}
