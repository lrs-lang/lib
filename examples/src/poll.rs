// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{time};
use std::poll::{self, POLL_READ};

fn main() {
    let timer = time::REAL.timer().unwrap();
    timer.interval(time::Time::seconds(5)).unwrap();

    let epoll = poll::Epoll::new().unwrap();
    epoll.add(&timer, POLL_READ).unwrap();

    let mut buf = [poll::EMPTY_EVENT; 20];
    println!("{:?}", epoll.wait(&mut buf));
}
