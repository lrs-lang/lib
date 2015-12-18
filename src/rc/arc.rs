// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, ptr, intrinsics};
use core::ptr::{NoAliasMemPtr, NoAliasObjPtr};
use core::iter::{IntoIterator};
use core::marker::{Leak, Unsize};
use core::ops::{CoerceUnsized};
use base::undef::{UndefState};
use atomic::{AtomicUsize};
use fmt::{Debug, Write};
use alloc::{self, MemPool};

struct Inner<T: ?Sized, H = alloc::Heap>
    where H: MemPool,
{
    count: AtomicUsize,
    pool: H,
    val: T,
}

/// A buffer used when creating a new `Arc`.
pub struct ArcBuf<T, Heap = alloc::Heap>
    where Heap: MemPool,
          T: Leak,
{
    data: NoAliasMemPtr<Inner<T, Heap>>,
}

impl<T, H> ArcBuf<T, H>
    where H: MemPool,
          T: Leak,
{
    /// Stores a value in the buffer, creating a real `Arc`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Arc<T, H> {
        unsafe {
            let data = self.data.get();
            intrinsics::forget(self);
            ptr::write(&mut (*data).val, val);
            Arc { data: NoAliasObjPtr::new(data) }
        }
    }
}

unsafe impl<T, H> Send for ArcBuf<T, H> where T: Leak, H: Send+MemPool { }
unsafe impl<T, H> Sync for ArcBuf<T, H> where T: Leak, H: MemPool { }

impl<T, H> Drop for ArcBuf<T, H>
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

/// An atomically reference-counted container.
pub struct Arc<T: ?Sized, Heap = alloc::Heap>
    where Heap: MemPool,
          T: Leak,
{
    data: NoAliasObjPtr<Inner<T, Heap>>,
}

impl<T: ?Sized, H = alloc::Heap> Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    /// Creates a new Arc.
    ///
    /// = Remarks
    ///
    /// This function first creates an `ArcBuf` which can then be used to create a real
    /// `Arc` as shown in the example. The function does not take a value argument itself
    /// since this would complicate handling the case where allocating memory fails.
    ///
    /// = Examples
    ///
    /// ----
    /// let arc: Arc<Vec<u8>> = try!(Arc::new()).set(Vec::new());
    /// ----
    pub fn new() -> Result<ArcBuf<T, H>>
        where H: OutOf,
              T: Sized,
    {
        Self::with_pool(H::out_of(()))
    }

    /// Creates a new Arc.
    ///
    /// = Remarks
    ///
    /// This function first creates an `ArcBuf` which can then be used to create a real
    /// `Arc` as shown in the example. The function does not take a value argument itself
    /// since this would complicate handling the case where allocating memory fails.
    ///
    /// = Examples
    ///
    /// ----
    /// let arc: Arc<Vec<u8>> = try!(Arc::new()).set(Vec::new());
    /// ----
    pub fn with_pool(mut pool: H) -> Result<ArcBuf<T, H>>
        where T: Sized,
    {
        unsafe {
            let data_ptr = try!(alloc::alloc::<Inner<T, H>, _>(&mut pool));
            ptr::write(&mut (*data_ptr).pool, pool);
            (*data_ptr).count.store(1);
            Ok(ArcBuf { data: NoAliasMemPtr::new(data_ptr) })
        }
    }

    /// Returns a mutable reference to the contained data if this is the only reference.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe {
            match self.data.count.load() {
                1 => Some(&mut (*self.data.get()).val),
                _ => None,
            }
        }
    }

    /// Adds a new reference, returning an `Arc` that points to the same data.
    pub fn add_ref(&self) -> Arc<T, H> {
        self.data.count.add(1);
        Arc { data: self.data }
    }
}

impl<T: ?Sized, U: ?Sized, H> CoerceUnsized<Arc<U, H>> for Arc<T, H>
    where T: Unsize<U> + Leak,
          U: Leak,
          H: MemPool,
{}

impl<'a, T, H> IntoIterator for &'a Arc<T, H>
    where &'a T: IntoIterator,
          H: MemPool,
          T: Leak,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (**self).into_iter() }
}

unsafe impl<T, H> UndefState for Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn num() -> usize { <NoAliasObjPtr<Inner<T, H>> as UndefState>::num() }

    unsafe fn set_undef(val: *mut Arc<T, H>, n: usize) {
        <NoAliasObjPtr<Inner<T, H>> as UndefState>::set_undef(&mut (*val).data, n);
    }

    unsafe fn is_undef(val: *const Arc<T, H>, n: usize) -> bool {
        <NoAliasObjPtr<Inner<T, H>> as UndefState>::is_undef(&(*val).data, n)
    }
}

unsafe impl<T: ?Sized, H> Send for Arc<T, H> where T: Sync+Send+Leak, H:Send+MemPool { }
unsafe impl<T: ?Sized, H> Sync for Arc<T, H> where T: Sync+Leak, H: MemPool { }

impl<T: ?Sized, H> Drop for Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            if self.data.count.sub(1) == 1 {
                if mem::needs_drop::<T>() {
                    ptr::drop(&mut (*self.data.get()).val);
                }
                let mut pool = ptr::read(&self.data.pool);
                alloc::free(&mut pool, self.data.get());
            }
        }
    }
}

impl<T: ?Sized, H> Deref for Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.data.val
    }
}

impl<T: ?Sized, H> To for Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn to(&self) -> Arc<T, H> {
        self.add_ref()
    }
}

impl<T: ?Sized, H> TryTo for Arc<T, H>
    where H: MemPool,
          T: Leak,
{
    fn try_to(&self) -> Result<Arc<T, H>> {
        Ok(self.add_ref())
    }
}

impl<T: ?Sized, H> Debug for Arc<T, H>
    where T: Debug + Leak,
          H: MemPool,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Arc {{ data: {:?} }}", self.deref())
    }
}
