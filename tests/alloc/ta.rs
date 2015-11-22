// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{TaAlloc, TaPool, Allocator};

#[test]
fn allocate_raw() {
    let mut buf = &mut [0; 5][..];
    let addr = buf.as_ptr() as usize;
    unsafe {
        let mut pool1 = TaPool::new(&mut buf);
        let mut pool2 = pool1;

        let a1 = TaAlloc::allocate_raw(&mut pool1, 1, 1).unwrap();
        test!(a1 as usize == addr);

        let a2 = TaAlloc::allocate_raw(&mut pool2, 2, 1).unwrap();
        test!(a2 as usize == addr + 1);

        let a3 = TaAlloc::allocate_raw(&mut pool1, 2, 1).unwrap();
        test!(a3 as usize == addr + 3);
    }
    test!(buf.as_ptr() as usize == addr + 5);
}

#[test]
fn reallocate_raw() {
    let mut buf = &mut [0; 5][..];
    unsafe {
        let mut pool = TaPool::new(&mut buf);
        let alloc = TaAlloc::allocate_raw(&mut pool, 1, 1).unwrap();
        *alloc = 1;
        let realloc = TaAlloc::reallocate_raw(&mut pool, alloc, 1, 2, 1).unwrap();
        test!(*realloc == 1);
    }
    test!(buf.len() == 3);
}
