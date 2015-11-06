// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fs};
use std::fs::flags::{UNMOUNT_LAZY};

fn main() {
    fs::unmount("hurr", UNMOUNT_LAZY).unwrap();
}
