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

/// An atomic object wrapper.
#[repr(C)]
pub struct Atomic<T>
    where T: Copy,
{
    val: Cell<T>,
}

unsafe impl<T> Sync for Atomic<T> where T: Copy+Send { }
unsafe impl<T> Send for Atomic<T> where T: Copy+Send { }

impl<T> Atomic<T>
    where T: Copy,
{
    /// Creates a new atomic object.
    ///
    /// [argument, val]
    /// The value which is initially stored in the atomic object.
    pub const fn new(val: T) -> Self {
        Atomic { val: Cell::new(val) }
    }

    pub unsafe fn wrap(val: *mut T) -> &'static Self {
        mem::cast(val)
    }

    pub unsafe fn as_ptr(&self) -> *mut T {
        self.val.ptr() as *mut _
    }

    /// Loads the value of the atomic object without ordering guarantees.
    pub fn load_unordered(&self) -> T {
        unsafe { intrinsics::atomic_load_unordered(self.val.ptr()) }
    }

    /// Loads the value of the atomic object with relaxed ordering guarantees.
    pub fn load_monotonic(&self) -> T {
        unsafe { intrinsics::atomic_load_relaxed(self.val.ptr()) }
    }

    /// Loads the value of the atomic object with acquire semantics.
    pub fn load_acquire(&self) -> T {
        unsafe { intrinsics::atomic_load_acq(self.val.ptr()) }
    }

    /// Loads the value of the atomic object with sequentially consistent
    /// semantics.
    pub fn load(&self) -> T {
        unsafe { intrinsics::atomic_load(self.val.ptr()) }
    }

    /// Stores a new value in the atomic object without ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    pub fn store_unordered(&self, val: T) {
        unsafe { intrinsics::atomic_store_unordered(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic object with relaxed ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    pub fn store_monotonic(&self, val: T) {
        unsafe { intrinsics::atomic_store_relaxed(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic object with release semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    pub fn store_release(&self, val: T) {
        unsafe { intrinsics::atomic_store_rel(self.val.ptr(), val) }
    }

    /// Stores a new value in the atomic object with sequentially consistent.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    pub fn store(&self, val: T) {
        unsafe { intrinsics::atomic_store(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic object by a new one with relaxed
    /// ordering guarantees.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_monotonic(&self, val: T) -> T {
        unsafe { intrinsics::atomic_xchg_relaxed(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic object by a new one with release
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_release(&self, val: T) -> T {
        unsafe { intrinsics::atomic_xchg_rel(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic object by a new one with acquire
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_acquire(&self, val: T) -> T {
        unsafe { intrinsics::atomic_xchg_acq(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic object by a new one with acquire-release
    /// semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange_acquire_release(&self, val: T) -> T {
        unsafe { intrinsics::atomic_xchg_acqrel(self.val.ptr(), val) }
    }

    /// Replaces the value in the atomic object by a new one with sequentially
    /// consistent semantics.
    ///
    /// [argument, val]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the previous value.
    pub fn exchange(&self, val: T) -> T {
        unsafe { intrinsics::atomic_xchg(self.val.ptr(), val) }
    }

    /// Conditionally replaces the value in the atomic object by a new one with
    /// relaxed ordering guarantees.
    ///
    /// [argument, old]
    /// The value the atomic object is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic object.
    ///
    /// = Description
    ///
    /// If the returned value is binary equal to `old`, the value in the atomic
    /// object has been replaced by `new`.
    pub fn compare_exchange_monotonic(&self, old: T, new: T) -> T {
        unsafe { intrinsics::atomic_cxchg_relaxed(self.val.ptr(), old, new) }
    }

    /// Conditionally replaces the value in the atomic object by a new one with
    /// release semantics.
    ///
    /// [argument, old]
    /// The value the atomic object is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic object.
    ///
    /// = Description
    ///
    /// If the returned value is binary equal to `old`, the value in the atomic
    /// object has been replaced by `new`.
    pub fn compare_exchange_release(&self, old: T, new: T) -> T {
        unsafe { intrinsics::atomic_cxchg_rel(self.val.ptr(), old, new) }
    }

    /// Conditionally replaces the value in the atomic object by a new one with
    /// acquire semantics.
    ///
    /// [argument, old]
    /// The value the atomic object is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic object.
    ///
    /// = Description
    ///
    /// If the returned value is binary equal to `old`, the value in the atomic
    /// object has been replaced by `new`.
    pub fn compare_exchange_acquire(&self, old: T, new: T) -> T {
        unsafe { intrinsics::atomic_cxchg_acq(self.val.ptr(), old, new) }
    }

    /// Conditionally replaces the value in the atomic object by a new one with
    /// acquire-release semantics.
    ///
    /// [argument, old]
    /// The value the atomic object is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic object.
    ///
    /// = Description
    ///
    /// If the returned value is binary equal to `old`, the value in the atomic
    /// object has been replaced by `new`.
    pub fn compare_exchange_acquire_release(&self, old: T, new: T) -> T {
        unsafe { intrinsics::atomic_cxchg_acqrel(self.val.ptr(), old, new) }
    }

    /// Conditionally replaces the value in the atomic object by a new one with
    /// sequentially consistent semantics.
    ///
    /// [argument, old]
    /// The value the atomic object is compared to.
    ///
    /// [argument, new]
    /// The value to be stored in the atomic object.
    ///
    /// [return_value]
    /// Returns the value previously stored in the atomic object.
    ///
    /// = Description
    ///
    /// If the returned value is binary equal to `old`, the value in the atomic
    /// object has been replaced by `new`.
    pub fn compare_exchange(&self, old: T, new: T) -> T {
        unsafe { intrinsics::atomic_cxchg(self.val.ptr(), old, new) }
    }
}

macro_rules! impl_atomic {
    ($name:ty, $raw:ident, $signed:expr) => {
        impl $name {
            /// Adds a value to the atomic integer with relaxed ordering guarantees.
            ///
            /// [argument, val]
            /// The value to be added to the integer.
            ///
            /// [return_value]
            /// Returns the old value.
            pub fn add_monotonic(&self, val: $raw) -> $raw {
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
            pub fn sub_monotonic(&self, val: $raw) -> $raw {
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
            pub fn and_monotonic(&self, val: $raw) -> $raw {
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
            pub fn or_monotonic(&self, val: $raw) -> $raw {
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
            pub fn nand_monotonic(&self, val: $raw) -> $raw {
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
            pub fn xor_monotonic(&self, val: $raw) -> $raw {
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
            pub fn min_monotonic(&self, val: $raw) -> $raw {
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
            pub fn max_monotonic(&self, val: $raw) -> $raw {
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

impl_atomic!(Atomic<u8>,    u8,    false);
impl_atomic!(Atomic<u16>,   u16,   false);
impl_atomic!(Atomic<u32>,   u32,   false);
impl_atomic!(Atomic<usize>, usize, false);
impl_atomic!(Atomic<i8>,    i8,    true);
impl_atomic!(Atomic<i16>,   i16,   true);
impl_atomic!(Atomic<i32>,   i32,   true);
impl_atomic!(Atomic<isize>, isize, true);

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl_atomic!(Atomic<u64>, u64, false);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl_atomic!(Atomic<i64>, i64, true);
