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
    UnixSockAddr, Socket, Type, MsgFlags, CMsgBuf,
};
use lrs::file::{File};
use lrs::fd::{FDContainer};
use lrs::time::{Real, Time};

fn main() {
    // Create socket
    let socket = Socket::new_unix(Type::Datagram).unwrap();

    // The destination of our message
    let mut addrbuf = [0; 128];
    let addr = UnixSockAddr::from_abstract(&mut addrbuf, "LRS_SOCKET").unwrap();

    // Send some fds
    let file = File::open_read("Makefile").unwrap();
    let mut cmsgbuf = [0; 128];
    let mut cmsg = CMsgBuf::new(&mut cmsgbuf);
    cmsg.fds(&[file.borrow()]);

    // The data we're sending
    let data = [&b"hello "[..], &b"world\n"[..]];

    // Done!
    loop {
        println!("{:?}", socket.send_msg(&data, addr.as_ref(), cmsg.as_ref(),
                                         MsgFlags::new()));
        Real.sleep_for(Time::milliseconds(200));
    }
}
