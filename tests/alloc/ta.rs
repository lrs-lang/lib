// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{TaPool, MemPool};

#[test]
fn alloc() {
    let mut buf = &mut [d8::new(0); 5][..];
    let addr = buf.as_ptr() as usize;
    unsafe {
        let mut pool1 = TaPool::new(&mut buf);
        let mut pool2 = pool1;

        let a1 = pool1.alloc(1, 1).unwrap();
        test!(a1 as usize == addr);

        let a2 = pool2.alloc(2, 1).unwrap();
        test!(a2 as usize == addr + 1);

        let a3 = pool1.alloc(2, 1).unwrap();
        test!(a3 as usize == addr + 3);
    }
    test!(buf.as_ptr() as usize == addr + 5);
}

#[test]
fn realloc() {
    let mut buf = &mut [d8::new(0); 5][..];
    unsafe {
        let mut pool = TaPool::new(&mut buf);
        let alloc = pool.alloc(1, 1).unwrap() as *mut u8;
        *alloc = 1;
        let realloc = pool.realloc(alloc as *mut d8, 1, 2, 1).unwrap() as *mut u8;
        test!(*realloc == 1);
    }
    test!(buf.len() == 3);
}
