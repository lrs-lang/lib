// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::thread::{scheduler, set_scheduler};

fn main() {
    let mut sched = scheduler(0).unwrap();
    sched.nice = 0;
    println!("{:?}", set_scheduler(0, sched));
    println!("{:?}", scheduler(0).unwrap().nice);
}
