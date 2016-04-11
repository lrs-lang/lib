// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_box"]
#![crate_type = "lib"]
#![feature(custom_derive)]
#![no_std]
#![allow(non_upper_case_globals)]

extern crate lrs_base  as base;
extern crate lrs_alloc as alloc;
extern crate lrs_fmt   as fmt;

use base::prelude::*;
use core::ptr::{NoAliasMutObjPtr, NoAliasMemPtr};
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
    data: NoAliasMemPtr<T>,
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
            let data = self.data.get();
            let pool = ptr::read(&self.pool);
            intrinsics::forget(self);
            ptr::write(data, val);
            Box {
                data: NoAliasMutObjPtr::new(data),
                pool: pool,
                _marker: PhantomData,
            }
        }
    }
}

impl<T, H> Drop for BoxBuf<T, H>
    where H: alloc::MemPool,
{
    fn drop(&mut self) {
        unsafe { alloc::free(&mut self.pool, self.data.get()); }
    }
}

/// A heap-allocated object.
pub struct Box<T: ?Sized, Heap = alloc::Heap>
    where Heap: alloc::MemPool,
{
    pool: Heap,
    _marker: PhantomData<T>,
    data: NoAliasMutObjPtr<T>,
}

impl<T, H> Box<T, H>
    where H: alloc::MemPool,
{
    /// Creates a new box.
    ///
    /// = See also
    ///
    /// * link:lrs::bx::Box::with_pool[with_pool]
    pub fn new() -> Result<BoxBuf<T, H>>
        where H: OutOf,
    {
        Self::with_pool(H::out_of(()))
    }

    /// Creates a new box.
    ///
    /// [argument, pool]
    /// The pool from which the box will be allocated.
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
    pub fn with_pool(mut pool: H) -> Result<BoxBuf<T, H>> {
        unsafe {
            let ptr = try!(alloc::alloc(&mut pool));
            Ok(BoxBuf { data: NoAliasMemPtr::new(ptr), pool: pool })
        }
    }

    /// Unwraps the contained value.
    pub fn into(mut self) -> T {
        unsafe {
            let val = ptr::read(&*self);
            alloc::free(&mut self.pool, self.data.get());
            mem::unsafe_forget(self);
            val
        }
    }
}

impl<T: ?Sized, H> Box<T, H>
    where H: alloc::MemPool,
{
    pub unsafe fn from_raw_parts(ptr: *mut T, pool: H) -> Box<T, H> {
        Box {
            pool: pool,
            data: NoAliasMutObjPtr::new(ptr),
            _marker: PhantomData,
        }
    }

    pub unsafe fn into_raw_parts(self) -> (*mut T, H) {
        let ptr = self.data.get();
        let pool = ptr::read(&self.pool);
        mem::unsafe_forget(self);
        (ptr, pool)
    }
}

impl<T, U, H1, H2> TryTo<Box<U, H2>> for Box<T, H1>
    where H1: alloc::MemPool,
          H2: alloc::MemPool+OutOf,
          T: TryTo<U>,
{
    fn try_to(&self) -> Result<Box<U, H2>> {
        let u = try!((**self).try_to());
        let bx = try!(Box::new());
        Ok(bx.set(u))
    }
}

impl<T: ?Sized, H> Deref for Box<T, H>
    where H: alloc::MemPool,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: ?Sized, H> DerefMut for Box<T, H>
    where H: alloc::MemPool,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: ?Sized, H> Drop for Box<T, H>
    where H: alloc::MemPool,
{
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                ptr::drop(self.data.get());
            }
            alloc::free(&mut self.pool, self.data.get());
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
