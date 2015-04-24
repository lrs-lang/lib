// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;
mod core { pub use linux::core::*; }
#[prelude_import] use linux::prelude::*;

use linux::string::{CPtrPtr};
use linux::process::{fork, exec};

fn main() {
    let pid = fork(|| {
        let mut buf = [0; 1024];
        let mut args = CPtrPtr::buffered(&mut buf);
        args.push("echo").unwrap();
        args.push("hello").unwrap();
        args.push("world").unwrap();
        let args = args.finish().unwrap();
        if let Err(e) = exec("echo", args) {
            println!("Error {:?}", e);
        }
    });
    println!("child pid: {:?}", pid);
}
