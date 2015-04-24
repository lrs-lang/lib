// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }

use linux::thread::{scoped};

const SIZE: usize = 1_000_000;

fn main() {
    let mut large_object = [1u8; SIZE];
    {
        let res = scoped(|| {
            println!("getting to work");
            for i in 0..SIZE {
                large_object[i] = 2;
            }
            println!("done working");
        });
        println!("joining");
        res.unwrap();
        println!("joined");
    }
    for i in 0..SIZE {
        assert!(large_object[i] == 2);
    }
}

// Scope compile fail:
// fn f() {
//     let guard = {
//         let obj = 1;
//         let mut builder = Builder::new().unwrap();
//         builder.scoped(|| {
//             let _ = &obj;
//         })
//     };
// }

// Leak compile fail:
// fn f() {
//     let obj = 1;
//     let mut builder = Builder::new().unwrap();
//     let res = builder.scoped(|| {
//         let _ = &obj;
//     });
//     let x = Rc::new(res);
// }
