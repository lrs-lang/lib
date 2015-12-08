// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{Bda, Allocator, empty_ptr};
use std::env::aux::{page_size};

fn ps() -> usize {
    page_size()
}

#[test]
fn allocate_raw() {
    unsafe {
        let alloc = Bda::allocate_raw(&mut (), 1, 1).unwrap();
        test!(alloc as usize % ps() == 0);
    }
}

#[test]
fn reallocate_raw() {
    unsafe {
        let alloc = Bda::allocate_raw(&mut (), 1, 1).unwrap();
        *alloc = 1;
        let realloc = Bda::reallocate_raw(&mut (), alloc, 1, ps()+1, 1).unwrap();
        test!(realloc as usize % ps() == 0);
        test!(*realloc == 1);
        *realloc.add(ps()) = 1;
    }
}

#[test]
fn free_raw() {
    unsafe {
        let alloc = Bda::allocate_raw(&mut (), 1, 1).unwrap();
        Bda::free_raw(&mut (), alloc, 1, 1);
    }
}

#[test]
#[should_panic]
fn segfault() {
    unsafe {
        let alloc = Bda::allocate_raw(&mut (), 1, 1).unwrap();
        Bda::free_raw(&mut (), alloc, 1, 1);
        *alloc = 1;
    }
}

#[test]
fn allocate() {
    unsafe {
        let alloc = Bda::allocate(&mut ()).unwrap();
        test!(alloc as usize % ps() == 0);
        *alloc = 1;

        test!(Bda::allocate::<()>(&mut ()).unwrap() == empty_ptr());
    }
}

#[test]
fn allocate_array() {
    unsafe {
        let alloc = Bda::allocate_array(&mut (), 2).unwrap();
        test!(alloc as usize % ps() == 0);
        *alloc = 1;
        *alloc.add(1) = 1;

        test!(Bda::allocate_array::<()>(&mut (), 2).unwrap() == empty_ptr());
    }
}

#[test]
fn reallocate_array() {
    unsafe {
        let alloc = Bda::allocate_array(&mut (), 1).unwrap();
        *alloc = 1;
        let realloc = Bda::reallocate_array(&mut (), alloc, 1, 2).unwrap();
        test!(realloc as usize % ps() == 0);
        test!(*realloc == 1);
        *realloc.add(1) = 1;
    }
}

#[test]
fn free_array() {
    unsafe {
        let alloc = Bda::allocate_array::<i32>(&mut (), 2).unwrap();
        Bda::free_array(&mut (), alloc, 2);
    }
}

#[test]
fn free() {
    unsafe {
        let alloc = Bda::allocate::<i32>(&mut ()).unwrap();
        Bda::free(&mut (), alloc);
    }
}
