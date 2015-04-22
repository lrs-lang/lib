// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::process::{fork, Command};

fn main() {
    let pid = fork(|| {
        let mut buf = [0; 1024];
        let mut cmd = Command::new(&mut buf);
        cmd.arg("-V");
        if let Err(e) = cmd.exec("/usr/local/bin/rustc") {
            println!("Error {:?}", e);
        }
    });
    println!("child pid: {}", pid);
    loop { }
}
