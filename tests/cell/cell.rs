// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::share::{Cell};

#[test]
fn test() {
    let cell = Cell::new(1);
    unsafe {
        test!(cell.get() == 1);
        test!(*cell.ptr() == 1);
        *cell.ptr() = 2;
        test!(cell.get() == 2);
        test!(*cell.ptr() == 2);
    }
}
