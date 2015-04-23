// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(unused_imports)]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::alloc::{LibcHeap, Bda, NoHeap, JeMalloc};

const NUM: usize = 10000000;

static mut BUF: [usize; NUM] = [0; NUM];

// Stats:
//
// libc / bda: 0.25s
// no heap: 0.3s
// jemalloc: 0.6s

fn main() {
    // let mut vec: Vec<usize, Bda> = vec!();
    let mut vec = Vec::buffered(unsafe { BUF.as_mut() });
    for i in 0..NUM {
        vec.push(i);
    }
    println!("size: {}", vec.len());
    println!("cap: {}", vec.capacity());
}
