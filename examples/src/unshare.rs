// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::thread::{unshare, scoped};
use std::process::clone::{CLONE_FS};
use std::env::{set_cwd, get_cwd};
use std::string::{NoNullString};

fn main() {
    scoped(|| {
        unshare(CLONE_FS).unwrap();
        set_cwd("/").unwrap();
        let mut bla: NoNullString = NoNullString::new();
        println!("{:?}", get_cwd(&mut bla));
    });
    let mut bla: NoNullString = NoNullString::new();
    println!("{:?}", get_cwd(&mut bla));
}
