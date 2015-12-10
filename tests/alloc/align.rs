// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{AlignAlloc, TaPool, MemPool};
use std::{mem};

#[test]
fn allocate_raw() {
    unsafe {
        let mut buf = [0u32; 4];
        let addr = mem::addr(&buf);
        let mut buf = buf.as_mut_bytes();
        let mut aa = AlignAlloc::<u32, _>::new(TaPool::new(&mut buf));
        let alloc = aa.alloc(1, 1).unwrap();
        test!(alloc as usize == addr);
        let alloc = aa.alloc(1, 1).unwrap();
        test!(alloc as usize == addr + 4);
    }
}
