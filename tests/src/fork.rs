// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::string::{CPtrPtr};
use std::process::{fork, exec, wait_all, wait_id, WAIT_EXITED, WAIT_DONT_REAP,
                     WAIT_DONT_BLOCK};

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
    println!("wait result: {:?}", wait_id(pid.unwrap(), WAIT_EXITED | WAIT_DONT_REAP));
    println!("wait result: {:?}", wait_all(WAIT_EXITED | WAIT_DONT_BLOCK));
}
