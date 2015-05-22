// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;

use lrs::user::{Information};
use lrs::alloc::{TaAlloc, TaPool};

fn main() {
    let mut buf = &mut [0; 64][..];
    let user: Information<TaAlloc> =
        Information::from_user_id_with_pool(1000, TaPool::new(&mut buf)).unwrap();
    println!("{:?}", user);
}
