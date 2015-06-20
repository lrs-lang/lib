// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;

use lrs::ringbuf::{DynRingBuf};

fn main() {
    let mut buf: DynRingBuf<_> = DynRingBuf::new();
    for i in 0..64 {
        if i % 2 == 0 {
            buf.push_left(i);
        } else {
            buf.push_right(i);
        }
    }
    println!("{:?}", buf);
    for el in &buf {
        print!("{:?}, ", el);
    }
}
