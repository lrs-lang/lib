// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_atomic"]
#![crate_type = "lib"]
#![feature(const_fn)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cell as cell;

use base::prelude::*;

use core::{mem, intrinsics};
use cell::cell::{Cell};

mod std { pub use ::base::std::*; }

/// Creates a release fence.
pub fn fence_release() {
    unsafe { intrinsics::atomic_fence_rel(); }
}

/// Creates an acquire fence.
pub fn fence_acquire() {
    unsafe { intrinsics::atomic_fence_acq(); }
}

/// Creates an acquire-release fence.
pub fn fence_acquire_release() {
    unsafe { intrinsics::atomic_fence_acqrel(); }
}

/// Creates a sequentially consistent fence.
pub fn fence() {
    unsafe { intrinsics::atomic_fence(); }
}

/// Creates a release fence.
pub fn single_thread_fence_release() {
    unsafe { intrinsics::atomic_singlethreadfence_rel(); }
}

/// Creates an acquire fence.
pub fn single_thread_fence_acquire() {
    unsafe { intrinsics::atomic_singlethreadfence_acq(); }
}

/// Creates an acquire-release fence.
pub fn single_thread_fence_acquire_release() {
    unsafe { intrinsics::atomic_singlethreadfence_acqrel(); }
}

/// Creates a sequentially consistent fence.
pub fn single_thread_fence() {
    unsafe { intrinsics::atomic_singlethreadfence(); }
}

macro_rules! impl_atomic {
    ($name:ident, $init:ident, $raw:ident, $signed:expr) => {
        /// An atomic integer wrapper.
        #[repr(C)]
        pub struct $name {
            val: Cell<$raw>,
        }

        unsafe impl Sync for $name { }
        unsafe impl Send for $name { }

        impl $name {
            /// Creates a new atomic integer.
            ///
            /// [argument, val]
            /// The value which is initially stored in the atomic integer.
            pub const fn new(val: $raw) -> $name {
                $name { val: Cell::new(val) }
            }

            pub unsafe fn wrap(val: *mut $raw) -> &'static $name {
                mem::cast(val)
            }

            pub unsafe fn as_ptr(&self) -> *mut $raw {
                self.val.ptr()
            }

            /// Loads the value of the atomic integer without ordering guarantees.
            pub fn load_unordered(&self) -> $raw {
                unsafe { intrinsics::atomic_load_unordered(self.val.ptr()) }
            }

            /// Loads the value of the atomic integer with relaxed ordering guarantees.
            pub fn load_weak(&self) -> $raw {
                unsafe { intrinsics::atomic_load_relaxed(self.val.ptr()) }
            }

            /// Loads the value of the atomic integer with acquire semantics.
            pub fn load_acquire(&self) -> $raw {
                unsafe { intrinsics::atomic_load_acq(self.val.ptr()) }
            }

            /// Loads the value of the atomic integer with sequentially consistent
            /// semantics.
            pub fn load(&self) -> $raw {
                unsafe { intrinsics::atomic_load(self.val.ptr()) }
            }

            /// Stores a new value in the atomic integer without ordering guarantees.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            pub fn store_unordered(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_unordered(self.val.ptr(), val) }
            }

            /// Stores a new value in the atomic integer with relaxed ordering guarantees.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            pub fn store_weak(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_relaxed(self.val.ptr(), val) }
            }

            /// Stores a new value in the atomic integer with release semantics.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            pub fn store_release(&self, val: $raw) {
                unsafe { intrinsics::atomic_store_rel(self.val.ptr(), val) }
            }

            /// Stores a new value in the atomic integer with sequentially consistent.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            pub fn store(&self, val: $raw) {
                unsafe { intrinsics::atomic_store(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by a new one with relaxed
            /// ordering guarantees.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the previous value.
            pub fn exchange_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_relaxed(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by a new one with release
            /// semantics.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the previous value.
            pub fn exchange_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_rel(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by a new one with acquire
            /// semantics.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the previous value.
            pub fn exchange_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_acq(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by a new one with acquire-release
            /// semantics.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the previous value.
            pub fn exchange_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg_acqrel(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by a new one with sequentially
            /// consistent semantics.
            ///
            /// [argument, val]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the previous value.
            pub fn exchange(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xchg(self.val.ptr(), val) }
            }

            /// Conditionally replaces the value in the atomic integer by a new one with
            /// relaxed ordering guarantees.
            ///
            /// [argument, old]
            /// The value the atomic integer is compared to.
            ///
            /// [argument, new]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the value previously stored in the atomic integer.
            ///
            /// = Remarks
            ///
            /// If the returned value is the same as `old`, the value in the atomic
            /// integer has been replaced by `new`.
            pub fn compare_exchange_weak(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_relaxed(self.val.ptr(), old, new) }
            }

            /// Conditionally replaces the value in the atomic integer by a new one with
            /// release semantics.
            ///
            /// [argument, old]
            /// The value the atomic integer is compared to.
            ///
            /// [argument, new]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the value previously stored in the atomic integer.
            ///
            /// = Remarks
            ///
            /// If the returned value is the same as `old`, the value in the atomic
            /// integer has been replaced by `new`.
            pub fn compare_exchange_release(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_rel(self.val.ptr(), old, new) }
            }

            /// Conditionally replaces the value in the atomic integer by a new one with
            /// acquire semantics.
            ///
            /// [argument, old]
            /// The value the atomic integer is compared to.
            ///
            /// [argument, new]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the value previously stored in the atomic integer.
            ///
            /// = Remarks
            ///
            /// If the returned value is the same as `old`, the value in the atomic
            /// integer has been replaced by `new`.
            pub fn compare_exchange_acquire(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_acq(self.val.ptr(), old, new) }
            }

            /// Conditionally replaces the value in the atomic integer by a new one with
            /// acquire-release semantics.
            ///
            /// [argument, old]
            /// The value the atomic integer is compared to.
            ///
            /// [argument, new]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the value previously stored in the atomic integer.
            ///
            /// = Remarks
            ///
            /// If the returned value is the same as `old`, the value in the atomic
            /// integer has been replaced by `new`.
            pub fn compare_exchange_acquire_release(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg_acqrel(self.val.ptr(), old, new) }
            }

            /// Conditionally replaces the value in the atomic integer by a new one with
            /// sequentially consistent semantics.
            ///
            /// [argument, old]
            /// The value the atomic integer is compared to.
            ///
            /// [argument, new]
            /// The value to be stored in the atomic integer.
            ///
            /// [return_value]
            /// Returns the value previously stored in the atomic integer.
            ///
            /// = Remarks
            ///
            /// If the returned value is the same as `old`, the value in the atomic
            /// integer has been replaced by `new`.
            pub fn compare_exchange(&self, old: $raw, new: $raw) -> $raw {
                unsafe { intrinsics::atomic_cxchg(self.val.ptr(), old, new) }
            }

            /// Adds a value to the atomic integer with relaxed ordering guarantees.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_relaxed(self.val.ptr(), val) }
            }

            /// Adds a value to the atomic integer with release semantics.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_rel(self.val.ptr(), val) }
            }

            /// Adds a value to the atomic integer with acquire semantics.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_acq(self.val.ptr(), val) }
            }

            /// Adds a value to the atomic integer with acquire-release semantics.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd_acqrel(self.val.ptr(), val) }
            }

            /// Adds a value to the atomic integer with sequentially consistent semantics.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xadd(self.val.ptr(), val) }
            }

            /// Subtracts a value from the atomic integer with relaxed ordering
            /// guarantees.
            ///
            /// [argument, val]
            /// The value to be subtracted from the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn sub_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_relaxed(self.val.ptr(), val) }
            }

            /// Subtracts a value from the atomic integer with release semantics.
            ///
            /// [argument, val]
            /// The value to be subtracted from the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn sub_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_rel(self.val.ptr(), val) }
            }

            /// Subtracts a value from the atomic integer with acquire semantics.
            ///
            /// [argument, val]
            /// The value to be subtracted from the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn sub_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_acq(self.val.ptr(), val) }
            }

            /// Subtracts a value from the atomic integer with acquire-release semantics.
            ///
            /// [argument, val]
            /// The value to be subtracted from the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn sub_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub_acqrel(self.val.ptr(), val) }
            }

            /// Subtracts a value from the atomic integer with sequentially consistent
            /// semantics.
            ///
            /// [argument, val]
            /// The value to be subtracted from the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn sub(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xsub(self.val.ptr(), val) }
            }

            /// Performs a binary and operation on the atomic integer with relaxed
            /// ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be binarily added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn and_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_relaxed(self.val.ptr(), val) }
            }

            /// Performs a binary and operation on the atomic integer with release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn and_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_rel(self.val.ptr(), val) }
            }

            /// Performs a binary and operation on the atomic integer with acquire
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn and_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_acq(self.val.ptr(), val) }
            }

            /// Performs a binary and operation on the atomic integer with acquire-release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn and_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and_acqrel(self.val.ptr(), val) }
            }

            /// Performs a binary and operation on the atomic integer with sequentially
            /// consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn and(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_and(self.val.ptr(), val) }
            }

            /// Performs a binary or operation on the atomic integer with relaxed
            /// ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be binarily or'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn or_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_relaxed(self.val.ptr(), val) }
            }

            /// Performs a binary or operation on the atomic integer with release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily or'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn or_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_rel(self.val.ptr(), val) }
            }

            /// Performs a binary or operation on the atomic integer with acquire
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily or'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn or_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_acq(self.val.ptr(), val) }
            }

            /// Performs a binary or operation on the atomic integer with acquire-release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily or'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn or_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or_acqrel(self.val.ptr(), val) }
            }

            /// Performs a binary or operation on the atomic integer with sequentially
            /// consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily or'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn or(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_or(self.val.ptr(), val) }
            }

            /// Performs a binary nand operation on the atomic integer with relaxed
            /// ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be binarily nand'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn nand_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_relaxed(self.val.ptr(), val) }
            }

            /// Performs a binary nand operation on the atomic integer with release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily nand'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn nand_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_rel(self.val.ptr(), val) }
            }

            /// Performs a binary nand operation on the atomic integer with acquire
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily nand'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn nand_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_acq(self.val.ptr(), val) }
            }

            /// Performs a binary nand operation on the atomic integer with
            /// acquire-release semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily nand'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn nand_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand_acqrel(self.val.ptr(), val) }
            }

            /// Performs a binary nand operation on the atomic integer with sequentially
            /// consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily nand'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn nand(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_nand(self.val.ptr(), val) }
            }

            /// Performs a binary xor operation on the atomic integer with relaxed
            /// ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be binarily xor'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn xor_weak(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_relaxed(self.val.ptr(), val) }
            }

            /// Performs a binary xor operation on the atomic integer with release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily xor'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn xor_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_rel(self.val.ptr(), val) }
            }

            /// Performs a binary xor operation on the atomic integer with acquire
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily xor'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn xor_acquire(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_acq(self.val.ptr(), val) }
            }

            /// Performs a binary xor operation on the atomic integer with acquire-release
            /// semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily xor'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn xor_acquire_release(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor_acqrel(self.val.ptr(), val) }
            }

            /// Performs a binary xor operation on the atomic integer with sequentially
            /// consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be binarily xor'd to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn xor(&self, val: $raw) -> $raw {
                unsafe { intrinsics::atomic_xor(self.val.ptr(), val) }
            }

            /// Replaces the value in the atomic integer by the minimum of the current
            /// value and a given value with relaxed ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn min_weak(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_relaxed(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_relaxed(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the minimum of the current
            /// value and a given value with release semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn min_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_rel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_rel(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the minimum of the current
            /// value and a given value with acquire semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn min_acquire(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_acq(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_acq(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the minimum of the current
            /// value and a given value with acquire-release semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn min_acquire_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min_acqrel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin_acqrel(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the minimum of the current
            /// value and a given value with sequentially consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn min(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_min(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umin(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the maximum of the current
            /// value and a given value with relaxed ordering guarantees.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn max_weak(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_relaxed(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_relaxed(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the maximum of the current
            /// value and a given value with release semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn max_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_rel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_rel(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the maximum of the current
            /// value and a given value with acquire semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn max_acquire(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_acq(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_acq(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the maximum of the current
            /// value and a given value with acquire-release semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn max_acquire_release(&self, val: $raw) -> $raw {
                unsafe {
                    if $signed {
                        intrinsics::atomic_max_acqrel(self.val.ptr(), val)
                    } else {
                        intrinsics::atomic_umax_acqrel(self.val.ptr(), val)
                    }
                }
            }

            /// Replaces the value in the atomic integer by the maximum of the current
            /// value and a given value with sequentially consistent semantics.
            ///
            /// [argument, val]
            /// The value that will be compared to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn max(&self, val: $raw) -> $raw {
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
impl_atomic!(AtomicUsize, ATOMIC_USIZE_INIT, usize, false);
impl_atomic!(AtomicI8,    ATOMIC_I8_INIT,    i8,    true);
impl_atomic!(AtomicI16,   ATOMIC_I16_INIT,   i16,   true);
impl_atomic!(AtomicI32,   ATOMIC_I32_INIT,   i32,   true);
impl_atomic!(AtomicIsize, ATOMIC_ISIZE_INIT, isize, true);

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl_atomic!(AtomicU64,   ATOMIC_U64_INIT,   u64,   false);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl_atomic!(AtomicI64,   ATOMIC_I64_INIT,   i64,   true);

/// Atomic `c_int`.
pub type AtomicCInt = AtomicI32;

/// An atomic pointer wrapper.
#[repr(C)]
pub struct AtomicPtr<T> {
    val: Cell<*const T>,
}

unsafe impl<T> Sync for AtomicPtr<T> { }
unsafe impl<T> Send for AtomicPtr<T> { }

impl<T> AtomicPtr<T> {
    /// Creates a new atomic pointer.
    ///
    /// [argument, val]
    /// The value which is initially stored in the atomic pointer.
    pub const fn new(val: *const T) -> Self {
        AtomicPtr { val: Cell::new(val) }
    }

    pub unsafe fn wrap(val: *mut *const T) -> &'static Self {
        mem::cast(val)
    }

    pub unsafe fn as_ptr(&self) -> *mut *mut T {
        self.val.ptr() as *mut _
    }

    /// Loads the value of the atomic pointer without ordering guarantees.
    pub fn load_unordered(&self) -> *mut T {
        unsafe { intrinsics::atomic_load_unordered(self.val.ptr()) as *mut T } 
    }

    /// Loads the value of the atomic pointer with relaxed ordering guarantees.
    pub fn load_weak(&self) -> *mut T {
        unsafe { intrinsics::atomic_load_relaxed(self.val.ptr()) as *mut T }
    }

    /// Loads the value of the atomic pointer with acquire semantics.
    pub fn load_acquire(&self) -> *mut T {
        unsafe { intrinsics::atomic_load_acq(self.val.ptr()) as *mut T }
    }

    /// Loads the value of the atomic pointer with sequentially consistent
    /// semantics.
    pub fn load(&self) -> *mut T {
        unsafe { intrinsics::atomic_load(self.val.ptr()) as *mut T }
    }

    /// Stores a new value in the atomic pointer without ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    pub fn store_unordered(&self, val: *const T) {
        unsafe { intrinsics::atomic_store_unordered(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic pointer with relaxed ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    pub fn store_weak(&self, val: *const T) {
        unsafe { intrinsics::atomic_store_relaxed(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic pointer with release semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    pub fn store_release(&self, val: *const T) {
        unsafe { intrinsics::atomic_store_rel(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic pointer with sequentially consistent.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    pub fn store(&self, val: *const T) {
        unsafe { intrinsics::atomic_store(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic pointer by a new one with relaxed
    /// ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_weak(&self, val: *const T) -> *mut T {
        unsafe { intrinsics::atomic_xchg_relaxed(self.val.ptr(), val) as *mut T }
    }

    /// Replaces the value in the atomic pointer by a new one with release
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_release(&self, val: *const T) -> *mut T {
        unsafe { intrinsics::atomic_xchg_rel(self.val.ptr(), val) as *mut T }
    }

    /// Replaces the value in the atomic pointer by a new one with acquire
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_acquire(&self, val: *const T) -> *mut T {
        unsafe { intrinsics::atomic_xchg_acq(self.val.ptr(), val) as *mut T }
    }

    /// Replaces the value in the atomic pointer by a new one with acquire-release
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_acquire_release(&self, val: *const T) -> *mut T {
        unsafe { intrinsics::atomic_xchg_acqrel(self.val.ptr(), val) as *mut T }
    }

    /// Replaces the value in the atomic pointer by a new one with sequentially
    /// consistent semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange(&self, val: *const T) -> *mut T {
        unsafe { intrinsics::atomic_xchg(self.val.ptr(), val) as *mut T }
    }

    /// Conditionally replaces the value in the atomic pointer by a new one with
    /// relaxed ordering guarantees.
    ///
    /// [argument, old]
    /// The value the atomic pointer is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic pointer.
    ///
    /// = Remarks
    ///
    /// If the returned value is the same as `old`, the value in the atomic
    /// pointer has been replaced by `new`.
    pub fn compare_exchange_weak(&self, old: *const T, new: *const T) -> *mut T {
        unsafe { intrinsics::atomic_cxchg_relaxed(self.val.ptr(), old, new) as *mut T }
    }

    /// Conditionally replaces the value in the atomic pointer by a new one with
    /// release semantics.
    ///
    /// [argument, old]
    /// The value the atomic pointer is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic pointer.
    ///
    /// = Remarks
    ///
    /// If the returned value is the same as `old`, the value in the atomic
    /// pointer has been replaced by `new`.
    pub fn compare_exchange_release(&self, old: *const T, new: *const T) -> *mut T {
        unsafe { intrinsics::atomic_cxchg_rel(self.val.ptr(), old, new) as *mut T }
    }

    /// Conditionally replaces the value in the atomic pointer by a new one with
    /// acquire semantics.
    ///
    /// [argument, old]
    /// The value the atomic pointer is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic pointer.
    ///
    /// = Remarks
    ///
    /// If the returned value is the same as `old`, the value in the atomic
    /// pointer has been replaced by `new`.
    pub fn compare_exchange_acquire(&self, old: *const T, new: *const T) -> *mut T {
        unsafe { intrinsics::atomic_cxchg_acq(self.val.ptr(), old, new) as *mut T }
    }

    /// Conditionally replaces the value in the atomic pointer by a new one with
    /// acquire-release semantics.
    ///
    /// [argument, old]
    /// The value the atomic pointer is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic pointer.
    ///
    /// = Remarks
    ///
    /// If the returned value is the same as `old`, the value in the atomic
    /// pointer has been replaced by `new`.
    pub fn compare_exchange_acquire_release(&self, old: *const T, new: *const T) -> *mut T {
        unsafe { intrinsics::atomic_cxchg_acqrel(self.val.ptr(), old, new) as *mut T }
    }

    /// Conditionally replaces the value in the atomic pointer by a new one with
    /// sequentially consistent semantics.
    ///
    /// [argument, old]
    /// The value the atomic pointer is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic pointer.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic pointer.
    ///
    /// = Remarks
    ///
    /// If the returned value is the same as `old`, the value in the atomic
    /// pointer has been replaced by `new`.
    pub fn compare_exchange(&self, old: *const T, new: *const T) -> *mut T {
        unsafe { intrinsics::atomic_cxchg(self.val.ptr(), old, new) as *mut T }
    }
}
