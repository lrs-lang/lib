// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{self, NoMem, MemPool};

#[test]
fn allocate_raw() {
    unsafe {
        test!(NoMem::default().alloc(1, 1).is_err());
    }
}

#[test]
fn allocate() {
    unsafe {
        test!(alloc::alloc::<u8, _>(&mut NoMem::default()).is_err());
    }
}

#[test]
fn allocate_array() {
    unsafe {
        test!(alloc::alloc_array::<u8, _>(&mut NoMem::default(), 1).is_err());
    }
}
