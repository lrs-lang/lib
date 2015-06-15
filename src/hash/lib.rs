// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hash"]
#![crate_type = "lib"]
#![feature(plugin, no_std, const_fn, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_wrapping as wrapping;

mod lrs { pub use base::lrs::*; }

pub mod xx_hash;

// pub trait Hash {
//     fn hash<H: Hasher>(&self, h: H) -> H;
// }
// 
// pub trait Hasher {
//     pub fn write_u64(&mut self, val: u64);
// }
// 
// impl Hasher for u64 {
//     pub fn write_u64(&mut self, val: u64) {
//         // see include/linux/hash in the kernel tree
//         const GOLDEN_RATIO_PRIME_64: u64 = 0x9e37fffffffc0001;
// 
//         *self = *self * val;
//     }
// }
