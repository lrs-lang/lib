// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_box"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(non_upper_case_globals)] 

#[macro_use]
extern crate lrs_core  as core;
extern crate lrs_base  as base;
extern crate lrs_alloc as alloc;
extern crate lrs_fmt   as fmt;

#[prelude_import] use base::prelude::*;
use core::{ptr, mem, intrinsics};
use core::ops::{Deref, DerefMut};
use fmt::{Debug, Write};

mod lrs { pub use fmt::lrs::*; }

/// A heap-allocated object.
pub struct BoxBuf<T, Heap = alloc::Heap>
    where Heap: alloc::Allocator,
{
    data: *mut T,
    _marker: PhantomData<Heap>,
}

impl<T, H> BoxBuf<T, H>
    where H: alloc::Allocator,
{
    /// Stores a value in the buffer, creating a real `Box`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Box<T, H> {
        unsafe {
            let data = self.data;
            intrinsics::forget(self);
            ptr::write(data, val);
            Box { data: data, _marker: PhantomData }
        }
    }
}

impl<T, H> Drop for BoxBuf<T, H>
    where H: alloc::Allocator,
{
    fn drop(&mut self) {
        unsafe { H::free(self.data); }
    }
}

/// A heap-allocated object.
pub struct Box<T, Heap = alloc::Heap>
    where Heap: alloc::Allocator,
{
    data: *mut T,
    _marker: PhantomData<Heap>,
}

impl<T, H> Box<T, H>
    where H: alloc::Allocator,
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
            let ptr = try!(H::allocate());
            Ok(BoxBuf { data: ptr, _marker: PhantomData })
        }
    }
}

impl<T, H> Deref for Box<T, H>
    where H: alloc::Allocator,
{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T, H> DerefMut for Box<T, H>
    where H: alloc::Allocator,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T, H> Drop for Box<T, H>
    where H: alloc::Allocator,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                ptr::read(self.data);
            }
            H::free(self.data);
        }
    }
}

impl<T, H> Debug for Box<T, H>
    where H: alloc::Allocator,
          T: Debug
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}
