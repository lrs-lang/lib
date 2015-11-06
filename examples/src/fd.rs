// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::socket::{Socket};
use std::socket::flags::{SOCK_NONE};
use std::file::{File, Mode};
use std::file::flags::{FILE_DONT_BLOCK, FILE_READ_ONLY};

fn main() {
    let file = File::open_read("/etc/fstab").unwrap();
    let flags = file.description_flags().unwrap();
    println!("{:?} {:x}", flags, flags.0 as u64);

    let socket = Socket::unix_stream(SOCK_NONE).unwrap();
    let flags = socket.description_flags().unwrap();
    println!("{:?} {:x}", flags, flags.0 as u64);
}

