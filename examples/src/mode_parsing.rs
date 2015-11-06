// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::file::{Mode};
use std::string::{ByteString};

fn main() {
    let mode: Mode = "rwxrwxrwx".parse().unwrap();
    //println!("{:?}", format!("{}", mode));
    let buf: ByteString = format!("{:?}", mode);
    assert!(&buf == "rwxrwxrwx");
}
