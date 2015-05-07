// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
mod core { pub use lrs::core::*; }
#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;

use lrs::socket::{
    Credentials, CMsgBuf, CMsgIter, CMsg,
};

fn main() {
    let mut buf = [0; 1024];
    let mut cmsgbuf = CMsgBuf::new(&mut buf);

    cmsgbuf.fds(&[1, 2, 3, 4, 5]);
    cmsgbuf.fds(&[6, 2, 3, 4]);
    cmsgbuf.credentials(Credentials { process_id: 1, user_id: 2, group_id: 3 });
    cmsgbuf.fds(&[6, 2, 3, 4]);
    cmsgbuf.credentials(Credentials { process_id: 4, user_id: 5, group_id: 6 });
    cmsgbuf.fds(&[6, 3, 4]);

    let mut iter = cmsgbuf.iter();
    while let Some(cmsg) = iter.next() {
        match cmsg {
            CMsg::Fds(f) => println!("{:?}\n", f),
            CMsg::Credentials(c) => {
                println!("Process id: {}", c.process_id);
                println!("User id: {}", c.user_id);
                println!("Group id: {}\n", c.group_id)
            },
            _ => println!("other\n"),
        };
    }
}

