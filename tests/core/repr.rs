// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::repr::{Repr};

#[test]
fn slice() {
    let x: &[u8] = &[1, 2];
    let y = x.repr();
    assert!(y.len == 2);
    assert!(y.ptr == &x[0]);
}
