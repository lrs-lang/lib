// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::socket::{Socket};
use std::socket::cmsg::{CMsgBuf};
use std::socket::msg::{MSG_NONE};
use std::socket::flags::{SOCK_NONE};
use std::socket::unix::{UnixSockAddr};
use std::file::{File};
use std::fd::{FDContainer};
use std::time::{REAL, Time};

fn main() {
    // Create socket
    let socket = Socket::unix_datagram(SOCK_NONE).unwrap();

    // The destination of our message
    let mut addrbuf = [0; 128];
    let addr = UnixSockAddr::from_abstract(&mut addrbuf, "LRS_SOCKET").unwrap();

    // Send some fds
    let file = File::open_read("Makefile").unwrap();
    let mut cmsgbuf = [0; 128];
    let mut cmsg = CMsgBuf::buffered(&mut cmsgbuf);
    cmsg.fds(&[file.borrow()]);

    // The data we're sending
    let data = ["hello ".as_ref(), "world\n".as_ref()];

    // Done!
    loop {
        println!("{:?}", socket.send_ctrl_to(&data, &addr, &cmsg, MSG_NONE));
        REAL.sleep_for(Time::milliseconds(200));
    }
}
