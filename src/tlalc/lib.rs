// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_tlalc"]
#![crate_type = "lib"]
#![feature(no_std, link_llvm_intrinsics, thread_local, const_fn)]
#![no_std]

//! The tlalc allocator.
//!
//! = Definitions
//!
//! This section defines various terms used in the remained of the documentation.
//!
//! == object
//!
//! A continuous region of memory.
//!
//! == size of an object
//!
//! The extent of an object.
//!
//! == address of an object
//!
//! The lowest address pointing to an object.
//!
//! == pointer to an object
//!
//! A pointer that contains the address of an object.

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_thread as thread;
extern crate lrs_arch_fns as arch_fns;

mod std { pub use base::std::*; }

pub use cache::{Cache};

#[macro_use]
mod util;
mod sys;
mod chunk;
mod p;
mod cache;

const CHUNK_SIZE: usize = 0x200000;
const CHUNK_MASK: usize = CHUNK_SIZE - 1;
const BLOCK_SIZE: usize = 0x1000;
const BLOCK_MASK: usize = BLOCK_SIZE - 1;
const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE / BLOCK_SIZE; // 0x200 == 512
const BLOCK_SHIFT: usize = 12;
const CACHE_SIZE: usize = 4 * BLOCK_SIZE;
const MIN_ALLOC: usize = 0x10;
const MAX_SMALL: usize = 0x100;
const MAX_SMALL_SHIFT: usize = 8;
const LARGE_CLASS_SHIFT: usize = (MAX_SMALL / MIN_ALLOC) - MAX_SMALL_SHIFT - 1; // 7

pub fn usable_size(size: usize) -> usize {
    unsafe {
        if likely!(size <= MAX_SMALL) {
            align!(size, [%] MIN_ALLOC)
        } else if size <= BLOCK_SIZE {
            size.next_power_of_two()
        } else {
            align!(size, [%] BLOCK_SIZE)
        }
    }
}
