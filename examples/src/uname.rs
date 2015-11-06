// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::{sys};

fn main() {
    let mut strinfo = sys::StrInfo::new();
    strinfo.update().unwrap();
    println!("{:?}", strinfo);
}