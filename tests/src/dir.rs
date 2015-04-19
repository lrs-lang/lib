// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

fn main() {
    for entry in linux::dir::iter(".", None) {
        println!("{:?}", entry);
    }

    //linux::dir::walk(".", |entry| {
    //    println!("{:?}", entry);
    //    if entry.name.as_bytes()[0] != b'.' && entry.ty == linux::dir::Type::Directory {
    //        println!("RECURSING");
    //        linux::dir::WalkOp::Recurse
    //    } else {
    //        linux::dir::WalkOp::Continue
    //    }
    //});
}
