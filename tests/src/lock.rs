// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, const_fn)]
#![plugin(lrs_core_plugin)]

use std::sync::{Lock};

fn main() {
    static LOCK: Lock = Lock::new();
    let _t = LOCK.lock();
    LOCK.lock();
}
