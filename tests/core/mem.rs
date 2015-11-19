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
    test!(mem::zeroed::<T>().x == [0; 2]);
}

#[test]
fn as_bytes() {
    test!(mem::as_bytes(&mem::zeroed::<T>()) == &[0; 16][..]);
}

#[test]
fn as_mut_bytes() {
    test!(mem::as_bytes(&mut mem::zeroed::<T>()) == &mut [0; 16][..]);
}

#[test]
fn is_suitable_for() {
    test!(mem::is_suitable_for::<u64>([0u64; 1].as_ref()));
    test!(!mem::is_suitable_for::<u64>([0u8; 1].as_ref()));

    let val = [0u8; 9];
    if val.as_ptr() as usize & 7 != 0 {
        test!(!mem::is_suitable_for::<u64>(&val));
    } else {
        test!(!mem::is_suitable_for::<u64>(&val[1..]));
    }
}

#[test]
fn from_bytes() {
    test!(mem::from_bytes::<T>([0u64; 2].as_ref()).unwrap() == &mem::zeroed::<T>());
}

#[test]
fn from_mut_bytes() {
    test!(mem::from_mut_bytes::<T>([0u64; 2].as_mut()).unwrap() ==
            &mut mem::zeroed::<T>());
}

#[test]
fn copy_as() {
    unsafe {
        test!(mem::copy_as::<_, [u8; 8]>(&0u64) == [0; 8]);
    }
}

#[test]
fn copy() {
    let mut dst = [0u8; 4];
    test!(mem::copy(&mut dst, &[1; 8]) == 4);
    test!(dst == [1; 4]);
}

#[test]
fn swap() {
    let mut a = 0;
    let mut b = 1;
    mem::swap(&mut a, &mut b);
    test!(a == 1);
    test!(b == 0);
}

#[test]
fn replace() {
    let mut a = 0;
    test!(mem::replace(&mut a, 1) == 0);
    test!(a == 1);
}

#[test]
fn size_of() {
    test!(mem::size_of::<u64>() == 8);
}

#[test]
fn align_of() {
    test!(mem::align_of::<u32>() == 4);
}

#[test]
fn needs_drop() {
    test!(!mem::needs_drop::<u64>());
    test!(mem::needs_drop::<Vec<u64>>());
}

#[test]
fn as_slice() {
    test!(mem::as_slice(&1u8) == &[1][..]);
    test!(mem::as_mut_slice(&mut 1u8) == &mut [1][..]);
}

#[test]
fn align_for() {
    let x = [0; 16];
    for i in 0..x.len() {
        let y = mem::align_for::<u64>(&x[i..]);
        test!(y.len() < 8 || mem::is_suitable_for::<u64>(y));
    }
}
