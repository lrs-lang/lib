// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_atomic"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;

use core::{mem, intrinsics};
use core::cell::{Cell};

pub mod linux {
    pub use ::core::linux::*;
}

pub fn fence_release() {
    unsafe { intrinsics::atomic_fence_rel(); }
}

pub fn fence_acquire() {
    unsafe { intrinsics::atomic_fence_acq(); }
}

pub fn fence_acquire_release() {
    unsafe { intrinsics::atomic_fence_acqrel(); }
}

pub fn fence_seqcst() {
    unsafe { intrinsics::atomic_fence(); }
}

macro_rules! impl_atomic {
    ($name:ident, $init:ident, $raw:ident, $signed:expr) => {
        pub const $init: $name = $name { val: Cell { data: 0 } };

        #[repr(C)]
        #[derive(Copy)]
        pub struct $name {
            val: Cell<$raw>,
        }

        impl $name {
            pub fn new(val: $raw) -> $name {
                $name { val: Cell { data: val } }
            }

            pub unsafe fn wrap(val: &mut $raw) -> &$name {
                mem::cast(val)
            }

            pub unsafe fn unwrap(&self) -> &mut $raw {
                mem::cast(self)
            }

            pub fn load(&self) -> $raw {
                unsafe { intrinsics::atomic_load_unordered(self.val.ptr()) }
            }

            pub fn load_weak(&self) -> $raw {
                unsafe { intrinsics::atomic_load_relaxed(self.val.ptr()) }
            }

            pub fn load_acquire(&self) -> $raw {
                unsafe { intrinsics::atomic_load_acq(self.val.ptr()) }
            }

            pub fn load_seqcst(&self) -> $raw {
                unsafe { intrinsics::atomic_load(self.val.ptr()) }
            }

            pub fn store(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_unordered(self.val.ptr(), val) }
            }

            pub fn store_weak(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_relaxed(self.val.ptr(), val) }
            }

            pub fn store_release(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_rel(self.val.ptr(), val) }
            }

            pub fn store_seqcst(&self, val: $raw) {
                unsafe { intrinsics::atomic_store(self.val.ptr(), val) }
            }

            pub fn exchange_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_relaxed(self.val.ptr(), val) }
            }

            pub fn exchange_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_rel(self.val.ptr(), val) }
            }

            pub fn exchange_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_acq(self.val.ptr(), val) }
            }

            pub fn exchange_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_acqrel(self.val.ptr(), val) }
            }

            pub fn exchange_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg(self.val.ptr(), val) }
            }

            pub fn compare_exchange_weak(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_relaxed(self.val.ptr(), old, new) }
            }

            pub fn compare_exchange_release(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_rel(self.val.ptr(), old, new) }
            }

            pub fn compare_exchange_acquire(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_acq(self.val.ptr(), old, new) }
            }

            pub fn compare_exchange_acquire_release(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_acqrel(self.val.ptr(), old, new) }
            }

            pub fn compare_exchange_seqcst(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg(self.val.ptr(), old, new) }
            }

            pub fn add_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_relaxed(self.val.ptr(), val) }
            }

            pub fn add_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_rel(self.val.ptr(), val) }
            }

            pub fn add_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_acq(self.val.ptr(), val) }
            }

            pub fn add_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_acqrel(self.val.ptr(), val) }
            }

            pub fn add_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd(self.val.ptr(), val) }
            }

            pub fn sub_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_relaxed(self.val.ptr(), val) }
            }

            pub fn sub_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_rel(self.val.ptr(), val) }
            }

            pub fn sub_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_acq(self.val.ptr(), val) }
            }

            pub fn sub_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_acqrel(self.val.ptr(), val) }
            }

            pub fn sub_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub(self.val.ptr(), val) }
            }

            pub fn and_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_relaxed(self.val.ptr(), val) }
            }

            pub fn and_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_rel(self.val.ptr(), val) }
            }

            pub fn and_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_acq(self.val.ptr(), val) }
            }

            pub fn and_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_acqrel(self.val.ptr(), val) }
            }

            pub fn and_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and(self.val.ptr(), val) }
            }

            pub fn or_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_relaxed(self.val.ptr(), val) }
            }

            pub fn or_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_rel(self.val.ptr(), val) }
            }

            pub fn or_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_acq(self.val.ptr(), val) }
            }

            pub fn or_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_acqrel(self.val.ptr(), val) }
            }

            pub fn or_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or(self.val.ptr(), val) }
            }

            pub fn nand_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_relaxed(self.val.ptr(), val) }
            }

            pub fn nand_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_rel(self.val.ptr(), val) }
            }

            pub fn nand_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_acq(self.val.ptr(), val) }
            }

            pub fn nand_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_acqrel(self.val.ptr(), val) }
            }

            pub fn nand_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand(self.val.ptr(), val) }
            }

            pub fn xor_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_relaxed(self.val.ptr(), val) }
            }

            pub fn xor_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_rel(self.val.ptr(), val) }
            }

            pub fn xor_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_acq(self.val.ptr(), val) }
            }

            pub fn xor_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_acqrel(self.val.ptr(), val) }
            }

            pub fn xor_seqcst(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor(self.val.ptr(), val) }
            }

            pub fn min_weak(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_relaxed(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_relaxed(self.val.ptr(), val)
                    }
                }
            }

            pub fn min_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_rel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_rel(self.val.ptr(), val)
                    }
                }
            }

            pub fn min_acquire(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_acq(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_acq(self.val.ptr(), val)
                    }
                }
            }

            pub fn min_acquire_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_acqrel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_acqrel(self.val.ptr(), val)
                    }
                }
            }

            pub fn min_seqcst(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin(self.val.ptr(), val)
                    }
                }
            }

            pub fn max_weak(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_relaxed(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_relaxed(self.val.ptr(), val)
                    }
                }
            }

            pub fn max_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_rel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_rel(self.val.ptr(), val)
                    }
                }
            }

            pub fn max_acquire(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_acq(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_acq(self.val.ptr(), val)
                    }
                }
            }

            pub fn max_acquire_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_acqrel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_acqrel(self.val.ptr(), val)
                    }
                }
            }

            pub fn max_seqcst(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax(self.val.ptr(), val)
                    }
                }
            }
        }
    }
}

impl_atomic!(AtomicU8,    ATOMIC_U8_INIT,    u8,    false);
impl_atomic!(AtomicU16,   ATOMIC_U16_INIT,   u16,   false);
impl_atomic!(AtomicU32,   ATOMIC_U32_INIT,   u32,   false);
impl_atomic!(AtomicU64,   ATOMIC_U64_INIT,   u64,   false);
impl_atomic!(AtomicUsize, ATOMIC_USIZE_INIT, usize, false);
impl_atomic!(AtomicI8,    ATOMIC_I8_INIT,    i8,    true);
impl_atomic!(AtomicI16,   ATOMIC_I16_INIT,   i16,   true);
impl_atomic!(AtomicI32,   ATOMIC_I32_INIT,   i32,   true);
impl_atomic!(AtomicI64,   ATOMIC_I64_INIT,   i64,   true);
impl_atomic!(AtomicIsize, ATOMIC_ISIZE_INIT, isize, true);
