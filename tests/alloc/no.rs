// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{NoMem, Allocator};

#[test]
fn allocate_raw() {
    unsafe {
        test!(NoMem::allocate_raw(&mut (), 1, 1).is_err());
    }
}

#[test]
fn allocate() {
    unsafe {
        test!(NoMem::allocate::<u8>(&mut ()).is_err());
    }
}

#[test]
fn allocate_array() {
    unsafe {
        test!(NoMem::allocate_array::<u8>(&mut (), 1).is_err());
    }
}
