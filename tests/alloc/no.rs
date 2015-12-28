// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{self, Dummy, MemPool};

#[test]
fn allocate_raw() {
    unsafe {
        test!(Dummy::out_of(()).alloc(1, 1).is_err());
    }
}

#[test]
fn allocate() {
    unsafe {
        test!(alloc::alloc::<u8, _>(&mut Dummy::out_of(())).is_err());
    }
}

#[test]
fn allocate_array() {
    unsafe {
        test!(alloc::alloc_array::<u8, _>(&mut Dummy::out_of(()), 1).is_err());
    }
}
