// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Memory allocation
//!
//! = Description
//!
//! :allocator: link:lrs::alloc::Allocator[Allocator]
//!
//! This module contains memory allocators and the {allocator} trait implemented by them.
//!
//! = Remarks
//!
//! :max: link:lrs::alloc::MAX_SIZE[MAX_SIZE]
//!
//! The maximum size of an allocation is limited to the maximum value that can be
//! represented in an `isize`. This limit is checked by the allocators and allocation will
//! fail if a too-large allocation is requested. The limit is also available through the
//! {max} constant.
//!
//! == `Heap` and `FbHeap`
//!
//! :heap: link:lrs::alloc::Heap[Heap]
//! :fbheap: link:lrs::alloc::FbHeap[FbHeap]
//!
//! This module contains two type definitions that affect the default behavior of lrs. The
//! {heap} allocator is the default allocator used by types that have an allocator
//! argument. For example, `Vec<T>` is the same as `Vec<T, Heap>`. The {fbheap} allocator
//! is the allocator used by functions that don't allocate in the common case and fall
//! back to allocating if they have to. For example, `File::open` will try to construct a
//! null-terminated path argument on the stack but falls back to allocating with the
//! {fbheap} allocator if the path is too long.
//!
//! == Jemalloc
//!
//! The Jemalloc allocator is only available if lrs was compiled with the `jemalloc`
//! option.
//!
//! = Examples
//!
//! The following example performs a simple allocate-store-read-free operation.
//!
//! ----
//! unsafe {
//!     let mem: *mut u8 = try!(Bda::allocate());
//!     *mem = 1;
//!     println!("{}", *mem);
//!     Bda::free(mem);
//! }
//! ----

pub use lrs_alloc::{
    MAX_SIZE, empty_ptr, Allocator, Heap, FbHeap, Libc, NoMem, Bda, TaAlloc, TaPool,
};

#[cfg(jemalloc)]
pub use lrs_alloc::{
    JeMalloc,
};
