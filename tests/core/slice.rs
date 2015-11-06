// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{slice};

#[test]
fn from_ptr() {
    let x: &[u8] = &[1];
    let y: &[u8] = unsafe { slice::from_ptr(x.as_ptr(), 1) };
    assert!(x == y);
}

#[test]
fn len() {
    let x: &[u8] = &[];
    assert!(x.len() == 0);
    assert!([0].len() == 1);
    assert!([0, 0].len() == 2);
}

#[test]
fn as_ptr() {
    let x: &[u8] = &[1];
    assert!(x.as_ptr() == &x[0]);
}

#[test]
fn addr() {
    let mut x = &mut [0u64; 2];
    assert!(x.as_bytes().len() == 16);
    assert!(x.as_mut_bytes().len() == 16);
}
