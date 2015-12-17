// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {BLOCK_SIZE};

extern {
    #[link_name = "llvm.expect.i1"]
    pub fn expect_bool(b1: bool, b2: bool) -> bool;
}

#[allow(unused_unsafe)]
macro_rules! likely {
    ($e:expr) => {
        ::util::expect_bool($e, true)
    }
}

#[allow(unused_unsafe)]
macro_rules! unlikely {
    ($e:expr) => {
        ::util::expect_bool($e, false)
    }
}

pub unsafe fn slots_per_class(bin: usize) -> usize {
    const BS: usize = BLOCK_SIZE;
    static SLOTS: [usize; 20] = [
        BS / 0x10, BS / 0x20, BS / 0x30, BS / 0x40, BS / 0x50, BS / 0x60, BS / 0x70,
        BS / 0x80, BS / 0x90, BS / 0xA0, BS / 0xB0, BS / 0xC0, BS / 0xD0, BS / 0xE0,
        BS / 0xF0,
        BS / 0x100, BS / 0x200, BS / 0x400, BS / 0x800, BS / 0x1000,
    ];
    *SLOTS.as_ptr().add(bin)
}

// pub fn check_size(mut cur: POpt<Slot>, one_size: usize, total_size: usize) {
//     let mut real_size = 0;
//     while let Some(c) = *cur {
//         real_size += one_size;
//         cur = c.next;
//     }
//     assert!(real_size == total_size);
// }
