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
    let mut builder: CPtrPtr<linux::alloc::Bda> = CPtrPtr::new().unwrap();
    for i in 0..200000 {
        builder.push("hello");
        builder.push("world");
    }
    let mut ptr = match builder.finish() {
        Ok(ptr) => ptr,
        Err(e) => { println!("Error: {}", e.desc()); loop { } },
    };
    unsafe {
        for &addr in &ptr[..ptr.len()-1] {
            puts(addr);
        }
    }

    println!("{}", ptr.len());
}
