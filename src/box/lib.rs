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
use core::{ptr, mem};
use core::ops::{Deref, DerefMut};
use base::error::{Errno};
use fmt::{Debug, Write};

mod lrs { pub use fmt::lrs::*; }

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
    /// Creates a new box and stores a value in it.
    ///
    /// [argument, val]
    /// The value to be stored in the box.
    ///
    /// [return_value]
    /// On success, the box is returned, otherwise the error and the value are returned.
    pub fn new(val: T) -> Result<Box<T, H>, (T, Errno)> {
        unsafe {
            let ptr = match H::allocate() {
                Ok(p) => p,
                Err(e) => return Err((val, e)),
            };
            ptr::write(ptr, val);
            Ok(Box { data: ptr, _marker: PhantomData })
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
