// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::undef::{UndefState};
use std::{mem};

#[test]
fn bool() {
    test!(bool::num() == 254);
    test!(mem::size_of::<bool>() == 1);

    unsafe {
        let mut x = false as u8;
        for i in 0..bool::num() {
            test!(!bool::is_undef(&x as *const _ as *const _, i));
        }

        x = true as u8;
        for i in 0..bool::num() {
            test!(!bool::is_undef(&x as *const _ as *const _, i));
        }

        for i in 0..bool::num() {
            bool::set_undef(&mut x as *mut _ as *mut _, i);
            test!(bool::is_undef(&x as *const _ as *const _, i));
            test!(x != false as u8);
            test!(x != true as u8);
        }
    }
}

#[test]
fn char() {
    test!(char::num() == 0xE000 - 0xD800);
    test!(mem::size_of::<char>() == 4);

    unsafe {
        let mut x = 0u32;

        for i in 0..char::num() {
            char::set_undef(&mut x as *mut _ as *mut _, i);
            test!(char::is_undef(&x as *const _ as *const _, i));
            test!(x == 0xE000 + i as u32);
        }
    }
}

#[test]
fn ref_() {
    test!(<&u8>::num() == 4096);
    test!(<&mut u8>::num() == 4096);

    unsafe {
        let mut x = 0usize;
        for i in 0..(<&u8>::num()) {
            <&u8>::set_undef(&mut x as *mut _ as *mut _, i);
            test!(<&u8>::is_undef(&x as *const _ as *const _, i));
            test!(x == i);

            <&mut u8>::set_undef(&mut x as *mut _ as *mut _, i);
            test!(<&mut u8>::is_undef(&x as *const _ as *const _, i));
            test!(x == i);
        }
    }
}

#[test]
fn slice() {
    test!(<&[u8]>::num() == 4096);
    test!(<&mut [u8]>::num() == 4096);
    test!(mem::size_of::<&[u8]>() == mem::size_of::<[usize; 2]>());

    unsafe {
        let mut x = [0usize, 0usize];
        for i in 0..(<&[u8]>::num()) {
            <&[u8]>::set_undef(&mut x as *mut _ as *mut _, i);
            test!(<&[u8]>::is_undef(&x as *const _ as *const _, i));
            test!(x == [0, i] || x == [i, 0]);

            <&mut [u8]>::set_undef(&mut x as *mut _ as *mut _, i);
            test!(<&mut [u8]>::is_undef(&x as *const _ as *const _, i));
            test!(x == [0, i] || x == [i, 0]);
        }
    }
}

#[test]
fn str() {
    test!(<&str>::num() == 4096);
    test!(mem::size_of::<&str>() == mem::size_of::<[usize; 2]>());

    unsafe {
        let mut x = [0usize, 0usize];
        for i in 0..(<&str>::num()) {
            <&str>::set_undef(&mut x as *mut _ as *mut _, i);
            test!(<&str>::is_undef(&x as *const _ as *const _, i));
            test!(x == [0, i] || x == [i, 0]);
        }
    }
}
