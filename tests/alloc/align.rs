// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{AlignAlloc, TaPool, TaAlloc, Allocator};
use std::{mem};

#[test]
fn allocate_raw() {
    unsafe {
        let mut buf = [0u32; 4];
        let addr = mem::addr(&buf);
        let mut buf = buf.as_mut_bytes();
        let mut pool = TaPool::new(&mut buf);
        let alloc = AlignAlloc::<u32, TaAlloc>::allocate_raw(&mut pool, 1, 1).unwrap();
        test!(alloc as usize == addr);
        let alloc = AlignAlloc::<u32, TaAlloc>::allocate_raw(&mut pool, 1, 1).unwrap();
        test!(alloc as usize == addr + 4);
    }
}
