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
    let info = linux::fs::FileSystemInfo::from_path("/sys");
    //let file = linux::file::File::open_read("/").unwrap();
    //let info = file.fs_info();
    println!("{:?}", info);
}
