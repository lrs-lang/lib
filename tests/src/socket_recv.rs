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
    UnixSockAddr, Socket, Type, MsgFlags, CMsg,
};
use lrs::fd::{FDContainer};
use lrs::string::{AsByteStr};

fn main() {
    // Create socket
    let socket = Socket::new_unix(Type::Datagram).unwrap();

    // The addr we bind to
    let mut addr_buf = [0; 128];
    let addr = UnixSockAddr::from_abstract(&mut addr_buf, "LRS_SOCKET").unwrap();

    println!("{:?}", socket.bind(&addr));

    // Buffers for data, addr and cmsg
    let mut data_buf = [0; 128];
    let mut addr_buf = [0; 128];
    let mut cmsg_buf = [0; 128];

    // Done!
    loop {
        let rv = socket.recv_msg(&mut [&mut data_buf], &mut addr_buf, &mut cmsg_buf,
                                  MsgFlags::new());
        match rv {
            Ok((n, addr, iter)) => {
                println!("Received {:?} from {:?}", data_buf[..n].as_byte_str(), addr);
                println!("Additional cmsgs:");
                for cmsg in iter {
                    println!("{:?}", cmsg);
                }
                println!("")
            },
            Err(e) => println!("Error: {:?}", e),
        };
    }
}
