// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::socket::{Socket};
use std::socket::msg::{MSG_NONE};
use std::socket::unix::{UnixSockAddr};
use std::socket::flags::{SOCK_NONE};
use std::fd::{FDContainer};
use std::string::{AsByteStr};

fn main() {
    // Create socket
    let socket = Socket::unix_datagram(SOCK_NONE).unwrap();

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
                                 MSG_NONE);
        match rv {
            Ok((n, addr, iter, _)) => {
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
