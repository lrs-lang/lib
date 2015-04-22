// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Atomic integers
//!
//! This module contains integer wrappers with atomic operations. All types support the
//! same operations:
//!
//! - `new`: Creates a new object with the specified value.
//! - `wrap`: Wraps the integer as an atomic integer. The integer must be aligned and must
//!           not be accessed non-atomically concurrently or the behavior is undefined.
//! - `unwrap`: Returns a mutable pointer to the integer.
//! - `load`: Loads the value.
//! - `store`: Stores a new value.
//! - `exchange`: Stores a new value and returns the old one.
//! - `compare_exchange`: Compares the current value to a given one and if they match
//!                       replaces the value by by a new one. Returns the old value.
//! - `add`: Adds a value to the current value and returns the old value.
//! - `sub`, `and`, `or`, `nand`, `xor`: Like `add`.
//! - `min`: Stores the minimum of the current value and a new value. Returns the old
//!          value.
//! - `max`: Like `min`.
//!
//! The default ordering is sequentially consistent. The other available orderings are
//!
//! - `unordered`: No ordering guarantees but races with this mode are not undefined
//!                behavior.
//! - `weak`: Relaxed in C++11.
//! - `release`, `acquire`, `acquire_release`: As in C++11.
//!
//! See the C++11 standard for a concise description of these orderings.

pub use linux_atomic::{
    fence_release, fence_acquire, fence_acquire_release, fence, AtomicU8,
    ATOMIC_U8_INIT, AtomicU16,   ATOMIC_U16_INIT, AtomicU32,   ATOMIC_U32_INIT, AtomicU64,
    ATOMIC_U64_INIT, AtomicUsize, ATOMIC_USIZE_INIT, AtomicI8,    ATOMIC_I8_INIT,
    AtomicI16,   ATOMIC_I16_INIT, AtomicI32,   ATOMIC_I32_INIT, AtomicI64,
    ATOMIC_I64_INIT, AtomicIsize, ATOMIC_ISIZE_INIT, AtomicCInt, ATOMIC_CINT_INIT,
};
