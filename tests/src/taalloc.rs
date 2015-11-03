// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::user::{Information};
use std::alloc::{TaAlloc, TaPool};

fn main() {
    let mut buf = &mut [0; 64][..];
    let user: Information<TaAlloc> =
        Information::from_user_id_with_pool(1000, TaPool::new(&mut buf)).unwrap();
    println!("{:?}", user);
}
