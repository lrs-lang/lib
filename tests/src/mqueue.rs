// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

pub use std::file::flags::{FILE_READ_WRITE, FILE_CREATE};
pub use std::file::mode::{MODE_FILE};
pub use std::msg_queue::{MsgQueue};
pub use std::time::{REAL, Time};

fn main() {
    let flags = FILE_READ_WRITE | FILE_CREATE;
    let queue = MsgQueue::open("test", flags, MODE_FILE, None).unwrap();
    println!("{:?}", queue.description_flags());
    let timeout = REAL.get_time().unwrap() + Time::seconds(1);
    println!("{:?}", queue.send_timeout("hello, world".as_ref(), 1, timeout));
    // println!("{:?}", queue.recv(&mut [0; 8192]));
}
