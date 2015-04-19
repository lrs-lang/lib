// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::{time, poll};

fn main() {
    let timer = time::Real.timer().unwrap();
    timer.interval(time::Time::seconds(5)).unwrap();

    let epoll = poll::Epoll::new().unwrap();
    let mut flags = poll::Flags::new();
    flags.set_readable(true);
    epoll.add(&timer, flags).unwrap();

    let mut buf = [poll::EMPTY_EVENT; 20];
    println!("{:?}", epoll.wait(&mut buf));
}
