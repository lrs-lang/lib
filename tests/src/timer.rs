// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;

use linux::time::{self, Time};

fn main() {
    let timer = time::Real.timer().unwrap();
    timer.interval_in(Time::seconds(1), Time::seconds(5)).unwrap();

    time::Real.sleep_for(Time::seconds(10)).unwrap();

    println!("{:?}", timer.ticks().unwrap());
}
