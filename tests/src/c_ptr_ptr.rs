// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::string::{CPtrPtr};

extern {
    fn puts(p: *const i8);
}

fn main() {
    let mut buf = [0; 1024];
    let mut builder = CPtrPtr::new(&mut buf);
    for i in 0..20 {
        builder.push("hello");
        builder.push("world");
    }
    let mut ptr: *const *mut i8 = match builder.finish() {
        Ok(ptr) => ptr,
        Err(e) => { println!("Error: {}", e.desc()); loop { } },
    };
    unsafe {
        while !(*ptr).is_null() {
            puts(*ptr);
            ptr = ptr.add(1);
        }
    }
}
