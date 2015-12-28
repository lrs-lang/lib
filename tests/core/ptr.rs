// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{ptr, intrinsics};

#[test]
fn read() {
    unsafe {
        test!(ptr::read(&0) == 0);
    }
}

#[test]
fn write() {
    let mut x = 0;
    unsafe {
        ptr::write(&mut x, 1);
    }
    test!(x == 1);
}

#[test]
#[should_panic]
fn drop() {
    struct X;

    impl Drop for X {
        fn drop(&mut self) {
            abort!();
        }
    }
    
    let mut x = X;
    unsafe {
        ptr::drop(&mut x);
        intrinsics::forget(x);
    }
}

#[test]
fn memcpy() {
    let mut x = [0u8; 8];
    let y = [1u8; 8];
    unsafe {
        ptr::memcpy(x.as_mut_ptr(), y.as_ptr(), x.len());
    }
    test!(x == y);
}

#[test]
fn memmove() {
    let mut x = [0, 0, 0, 0, 1, 1, 1, 1];
    unsafe {
        ptr::memmove(x.as_mut_ptr(), x.as_mut_ptr().offset(4), 4);
    }
    test!(x == [1; 8]);
}

#[test]
fn is_null() {
    test!((0 as *const u8).is_null());
    test!(!(1 as *const u8).is_null());
    test!((0 as *mut u8).is_null());
    test!(!(1 as *mut u8).is_null());
}

#[test]
fn offset() {
    unsafe {
        test!((0 as *const u8).offset(1) as usize == 1);
        test!((0 as *mut u8).offset(1) as usize == 1);
    }
}

#[test]
fn add() {
    unsafe {
        test!((0 as *const u8).add(1) as usize == 1);
        test!((0 as *mut u8).add(1) as usize == 1);
    }
}

#[test]
fn sub() {
    unsafe {
        test!((1 as *const u8).sub(1) as usize == 0);
        test!((1 as *mut u8).sub(1) as usize == 0);
    }
}

#[test]
fn eq() {
    test!(1 as *const u8 == 1 as *const u8);
    test!(1 as *mut u8 == 1 as *mut u8);
}

#[test]
fn ord() {
    test!((1 as *const u8) < (2 as *const u8));
    test!((1 as *mut u8) < (2 as *mut u8));
}

#[test]
fn non_zero_eq() {
    unsafe {
        test!(ptr::NonZeroPtr::new(0 as *const u8)
                    == ptr::NonZeroPtr::new(0 as *const u8));
    }
}

#[test]
fn non_zero_ord() {
    unsafe {
        test!(ptr::NonZeroPtr::new(0 as *const u8)
                    < ptr::NonZeroPtr::new(1 as *const u8));
    }
}
