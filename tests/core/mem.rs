// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};
use std::ops::{Eq};

struct T { x: [u64; 2] }
impl Pod for T { }
impl Copy for T { }
impl Eq for T { fn eq(&self, o: &T) -> bool { self.x == o.x } }

#[test]
fn zeroed() {
    assert!(mem::zeroed::<T>().x == [0; 2]);
}

#[test]
fn as_bytes() {
    assert!(mem::as_bytes(&mem::zeroed::<T>()) == &[0; 16][..]);
}

#[test]
fn as_mut_bytes() {
    assert!(mem::as_bytes(&mut mem::zeroed::<T>()) == &mut [0; 16][..]);
}

#[test]
fn is_suitable_for() {
    assert!(mem::is_suitable_for::<u64>([0u64; 1].as_ref()));
    assert!(!mem::is_suitable_for::<u64>([0u8; 1].as_ref()));

    let val = [0u8; 9];
    if val.as_ptr() as usize & 7 != 0 {
        assert!(!mem::is_suitable_for::<u64>(&val));
    } else {
        assert!(!mem::is_suitable_for::<u64>(&val[1..]));
    }
}

#[test]
fn from_bytes() {
    assert!(mem::from_bytes::<T>([0u64; 2].as_ref()).unwrap() == &mem::zeroed::<T>());
}

#[test]
fn from_mut_bytes() {
    assert!(mem::from_mut_bytes::<T>([0u64; 2].as_mut()).unwrap() ==
            &mut mem::zeroed::<T>());
}

#[test]
fn copy_as() {
    unsafe {
        assert!(mem::copy_as::<_, [u8; 8]>(&0u64) == [0; 8]);
    }
}

#[test]
fn copy() {
    let mut dst = [0u8; 4];
    assert!(mem::copy(&mut dst, &[1; 8]) == 4);
    assert!(dst == [1; 4]);
}

#[test]
fn swap() {
    let mut a = 0;
    let mut b = 1;
    mem::swap(&mut a, &mut b);
    assert!(a == 1);
    assert!(b == 0);
}

#[test]
fn replace() {
    let mut a = 0;
    assert!(mem::replace(&mut a, 1) == 0);
    assert!(a == 1);
}

#[test]
fn size_of() {
    assert!(mem::size_of::<u64>() == 8);
}

#[test]
fn align_of() {
    assert!(mem::align_of::<u64>() == 8);
}

#[test]
fn needs_drop() {
    assert!(!mem::needs_drop::<u64>());
    assert!(mem::needs_drop::<Vec<u64>>());
}

#[test]
fn as_slice() {
    assert!(mem::as_slice(&1u8) == &[1][..]);
    assert!(mem::as_mut_slice(&mut 1u8) == &mut [1][..]);
}

#[test]
fn align_for() {
    let x = [0; 16];
    for i in 0..x.len() {
        let y = mem::align_for::<u64>(&x[i..]);
        assert!(y.len() < 8 || mem::is_suitable_for::<u64>(y));
    }
}
