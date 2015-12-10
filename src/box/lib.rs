// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_box"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(non_upper_case_globals)] 

extern crate lrs_base  as base;
extern crate lrs_alloc as alloc;
extern crate lrs_fmt   as fmt;

use base::prelude::*;
use core::ptr::{OwnedPtr};
use core::{ptr, mem, intrinsics};
use fmt::{Debug, Write};
use core::marker::{Unsize};
use core::ops::{CoerceUnsized};

mod std { pub use fmt::std::*; }

/// A heap-allocated object.
pub struct BoxBuf<T, Heap = alloc::Heap>
    where Heap: alloc::MemPool,
{
    pool: Heap,
    data: OwnedPtr<T>,
}

impl<T, H> BoxBuf<T, H>
    where H: alloc::MemPool,
{
    /// Stores a value in the buffer, creating a real `Box`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Box<T, H> {
        unsafe {
            let data = self.data;
            let pool = ptr::read(&self.pool);
            intrinsics::forget(self);
            ptr::write(*data, val);
            Box { data: data, pool: pool }
        }
    }
}

impl<T, H> Drop for BoxBuf<T, H>
    where H: alloc::MemPool,
{
    fn drop(&mut self) {
        unsafe { alloc::free(&mut self.pool, *self.data); }
    }
}

/// A heap-allocated object.
pub struct Box<T: ?Sized, Heap = alloc::Heap>
    where Heap: alloc::MemPool,
{
    pool: Heap,
    data: OwnedPtr<T>,
}

impl<T, H> Box<T, H>
    where H: alloc::MemPool+Default,
{
    /// Creates a new box.
    ///
    /// = Remarks
    ///
    /// This function first creates an `BoxBuf` which can then be used to create a real
    /// `Box` as shown in the example. The function does not take a value argument itself
    /// since this would complicate handling the case where allocating memory fails.
    /// 
    /// = Examples
    ///
    /// ----
    /// let bx: Box<u8> = try!(Box::new()).set(0);
    /// ----
    pub fn new() -> Result<BoxBuf<T, H>> {
        unsafe {
            let mut pool = H::default();
            let ptr = try!(alloc::alloc(&mut pool));
            Ok(BoxBuf { data: OwnedPtr::new(ptr), pool: pool })
        }
    }
}

impl<T: ?Sized, H> Deref for Box<T, H>
    where H: alloc::MemPool,
{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &**self.data }
    }
}

impl<T: ?Sized, H> DerefMut for Box<T, H>
    where H: alloc::MemPool,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut **self.data }
    }
}

impl<T: ?Sized, H> Drop for Box<T, H>
    where H: alloc::MemPool,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                ptr::drop(*self.data);
            }
            alloc::free(&mut self.pool, *self.data);
        }
    }
}

impl<T: ?Sized, H> Debug for Box<T, H>
    where H: alloc::MemPool,
          T: Debug
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<Box<U>> for Box<T>
    where T: Unsize<U>,
{}
