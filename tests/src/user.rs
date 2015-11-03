// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::user::{self};

fn main() {
    let mut buf = [0; user::INFO_BUF_SIZE];
    let mut err = Ok(());
    {
        let mut iter = user::iter_buf(Some(&mut err));
        while let Some(user) = iter.next(&mut buf) {
            println!("{:?}", user);
        }
    }
    println!("{:?}", err);
}
