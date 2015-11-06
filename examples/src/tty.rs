// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::time::{self, Time};
use std::tty::{Tty, is_a_tty, hang_up};
use std::{process};
use std::fd::{STDIN};
use std::file::flags::{FILE_READ_WRITE};

fn main() {
    println!("{}", process::process_id());
    let tty = Tty::from_borrowed(0);
    println!("{:?}", tty.attributes());
    println!("{}", is_a_tty(&STDIN));
    println!("{:?}", hang_up());
}
