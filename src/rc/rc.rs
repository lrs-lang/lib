// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ptr::{NoAliasMemPtr, NoAliasObjPtr};
use core::iter::{IntoIterator};
use core::marker::{Leak, Unsize};
use core::ops::{CoerceUnsized};
use core::{mem, ptr, intrinsics};
use base::undef::{UndefState};
use cell::{Cell};
use fmt::{Debug, Write};
use alloc::{self, MemPool};

struct Inner<T: ?Sized, H = alloc::ThreadHeap>
    where H: MemPool,
{
    count: Cell<usize>,
    pool: H,
    val: T,
}

/// A buffer used when creating a new `Rc`.
pub struct RcBuf<T, Heap = alloc::ThreadHeap>
    where Heap: MemPool,
          T: Leak,
{
    data: NoAliasMemPtr<Inner<T, Heap>>,
}

impl<T, H> RcBuf<T, H>
    where H: MemPool,
          T: Leak,
{
    /// Stores a value in the buffer, creating a real `Rc`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Rc<T, H> {
        unsafe {
            let data = self.data.get();
            intrinsics::forget(self);
            ptr::write(&mut (*data).val, val);
            Rc { data: NoAliasObjPtr::new(data) }
        }
    }
}

unsafe impl<T, H> Send for RcBuf<T, H> where T: Leak, H: Send+MemPool { }
unsafe impl<T, H> Sync for RcBuf<T, H> where T: Leak, H: MemPool { }

impl<T, H> Drop for RcBuf<T, H>
    where H: MemPool,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let mut pool = ptr::read(&(*self.data.get()).pool);
            alloc::free(&mut pool, self.data.get());
        }
    }
}

/// A single-threaded reference-counted container.
pub struct Rc<T: ?Sized, Heap = alloc::ThreadHeap>
    where Heap: MemPool,
          T: Leak,
{
    data: NoAliasObjPtr<Inner<T, Heap>>,
}

impl<T: ?Sized, H = alloc::ThreadHeap> Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    /// Creates a new Rc.
    ///
    /// = Remarks
    ///
    /// This function first creates an `RcBuf` which can then be used to create a real
    /// `Rc` as shown in the example. The function does not take a value argument itself
    /// since this would complicate handling the case where allocating memory fails.
    ///
    /// = Examples
    ///
    /// ----
    /// let arc: Rc<Vec<u8>> = try!(Rc::new()).set(Vec::new());
    /// ----
    pub fn new() -> Result<RcBuf<T, H>>
        where H: OutOf,
              T: Sized,
    {
        Self::with_pool(H::out_of(()))
    }

    /// Creates a new Rc.
    ///
    /// = Remarks
    ///
    /// This function first creates an `RcBuf` which can then be used to create a real
    /// `Rc` as shown in the example. The function does not take a value argument itself
    /// since this would complicate handling the case where allocating memory fails.
    ///
    /// = Examples
    ///
    /// ----
    /// let arc: Rc<Vec<u8>> = try!(Rc::new()).set(Vec::new());
    /// ----
    pub fn with_pool(mut pool: H) -> Result<RcBuf<T, H>>
        where T: Sized,
    {
        unsafe {
            let data_ptr = try!(alloc::alloc::<Inner<T, H>, _>(&mut pool));
            ptr::write(&mut (*data_ptr).pool, pool);
            (*data_ptr).count.set(1);
            Ok(RcBuf { data: NoAliasMemPtr::new(data_ptr) })
        }
    }

    /// Returns a mutable reference to the contained data if this is the only reference.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe {
            match self.data.count.get() {
                1 => Some(&mut (*self.data.get()).val),
                _ => None,
            }
        }
    }

    /// Adds a new reference, returning an `Rc` that points to the same data.
    pub fn add_ref(&self) -> Rc<T, H> {
        self.data.count.set(self.data.count.get() + 1);
        Rc { data: self.data }
    }
}

impl<T: ?Sized, U: ?Sized, H> CoerceUnsized<Rc<U, H>> for Rc<T, H>
    where T: Unsize<U> + Leak,
          U: Leak,
          H: MemPool,
{}

impl<'a, T, H> IntoIterator for &'a Rc<T, H>
    where &'a T: IntoIterator,
          H: MemPool,
          T: Leak,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (**self).into_iter() }
}

unsafe impl<T, H> UndefState for Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn num() -> usize { <NoAliasObjPtr<Inner<T, H>> as UndefState>::num() }

    unsafe fn set_undef(val: *mut Rc<T, H>, n: usize) {
        <NoAliasObjPtr<Inner<T, H>> as UndefState>::set_undef(&mut (*val).data, n);
    }

    unsafe fn is_undef(val: *const Rc<T, H>, n: usize) -> bool {
        <NoAliasObjPtr<Inner<T, H>> as UndefState>::is_undef(&(*val).data, n)
    }
}

impl<T: ?Sized, H> !Send for Rc<T, H> { }

impl<T: ?Sized, H> Drop for Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let count = self.data.count.get();
            self.data.count.set(count - 1);
            if count == 1 {
                if mem::needs_drop::<T>() {
                    ptr::drop(&mut (*self.data.get()).val);
                }
                let mut pool = ptr::read(&self.data.pool);
                alloc::free(&mut pool, self.data.get());
            }
        }
    }
}

impl<T: ?Sized, H> Deref for Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.data.val
    }
}

impl<T: ?Sized, H> From for Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn from(t: &Rc<T, H>) -> Self {
        t.add_ref()
    }
}

impl<T: ?Sized, H> TryFrom for Rc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn try_from(t: &Rc<T, H>) -> Result<Self> {
        Ok(t.add_ref())
    }
}

impl<T: ?Sized, H> Debug for Rc<T, H>
    where T: Debug + Leak,
          H: MemPool,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Rc {{ data: {:?} }}", self.deref())
    }
}
