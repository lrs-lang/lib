// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

pub use lrs::file::flags::{FILE_READ_WRITE, FILE_CREATE};
pub use lrs::file::mode::{MODE_FILE};
pub use lrs::msg_queue::{MsgQueue};
pub use lrs::time::{REAL, Time};

fn main() {
    let flags = FILE_READ_WRITE | FILE_CREATE;
    let queue = MsgQueue::open("test", flags, MODE_FILE, None).unwrap();
    println!("{:?}", queue.description_flags());
    let timeout = REAL.get_time().unwrap() + Time::seconds(1);
    println!("{:?}", queue.send_timeout("hello, world".as_ref(), 1, timeout));
    // println!("{:?}", queue.recv(&mut [0; 8192]));
}
