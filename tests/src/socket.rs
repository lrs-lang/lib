// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;

use lrs::socket::{UnixSockAddr};

fn main() {
    let mut buf = [0; 128];
    {
        let addr = UnixSockAddr::new_path(&mut buf, "/tmp/socket").unwrap();
        println!("{:?}", addr);
    }
    {
        let addr = UnixSockAddr::new_unnamed(&mut buf).unwrap();
        println!("{:?}", addr);
    }
    {
        let addr = UnixSockAddr::new_abstract(&mut buf, "hurr\0durr").unwrap();
        println!("{:?}", addr);
    }
}
