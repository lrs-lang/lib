// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{self, Bda, MemPool, empty_ptr};
use std::env::aux::{page_size};

fn ps() -> usize {
    page_size()
}

#[test]
fn allocate_raw() {
    unsafe {
        let alloc = Bda.alloc(1, 1).unwrap();
        test!(alloc as usize % ps() == 0);
    }
}

#[test]
fn reallocate_raw() {
    unsafe {
        let alloc = Bda.alloc(1, 1).unwrap();
        *alloc = 1;
        let realloc = Bda.realloc(alloc, 1, ps()+1, 1).unwrap();
        test!(realloc as usize % ps() == 0);
        test!(*realloc == 1);
        *realloc.add(ps()) = 1;
    }
}

#[test]
fn free_raw() {
    unsafe {
        let alloc = Bda.alloc(1, 1).unwrap();
        Bda.free(alloc, 1, 1);
    }
}

#[test]
#[should_panic]
fn segfault() {
    unsafe {
        let alloc = Bda.alloc(1, 1).unwrap();
        Bda.free(alloc, 1, 1);
        *alloc = 1;
    }
}

#[test]
fn allocate() {
    unsafe {
        let alloc = alloc::alloc(&mut Bda).unwrap();
        test!(alloc as usize % ps() == 0);
        *alloc = 1;

        test!(alloc::alloc::<(), _>(&mut Bda).unwrap() == empty_ptr());
    }
}

#[test]
fn allocate_array() {
    unsafe {
        let alloc = alloc::alloc_array(&mut Bda, 2).unwrap();
        test!(alloc as usize % ps() == 0);
        *alloc = 1;
        *alloc.add(1) = 1;

        test!(alloc::alloc_array::<(), _>(&mut Bda, 2).unwrap() == empty_ptr());
    }
}

#[test]
fn reallocate_array() {
    unsafe {
        let alloc = alloc::alloc_array(&mut Bda, 1).unwrap();
        *alloc = 1;
        let realloc = alloc::realloc_array(&mut Bda, alloc, 1, 2).unwrap();
        test!(realloc as usize % ps() == 0);
        test!(*realloc == 1);
        *realloc.add(1) = 1;
    }
}

#[test]
fn free_array() {
    unsafe {
        let alloc = alloc::alloc_array::<i32, _>(&mut Bda, 2).unwrap();
        alloc::free_array(&mut Bda, alloc, 2);
    }
}

#[test]
fn free() {
    unsafe {
        let alloc = alloc::alloc::<i32, _>(&mut Bda).unwrap();
        alloc::free(&mut Bda, alloc);
    }
}
