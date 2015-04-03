// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate linux;

use linux::file::{can_access};

fn main() {
    assert_eq!(can_access("Makefile", "rw-".parse().unwrap()), Ok(true));
    assert_eq!(can_access("Makefile", "--x".parse().unwrap()), Ok(false));
}
