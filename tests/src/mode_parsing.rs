// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate linux;

use linux::file::{Mode};

fn main() {
    let mode: Mode = "rwxrwxrwx".parse().unwrap();
    assert_eq!(mode.to_string(), "rwxrwxrwx");
}
