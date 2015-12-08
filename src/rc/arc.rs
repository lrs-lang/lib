// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, ptr, intrinsics};
use core::ptr::{OwnedPtr};
use core::iter::{IntoIterator};
use core::marker::{Leak, Unsize};
use core::ops::{CoerceUnsized};
use base::clone::{Clone, MaybeClone};
use base::default::{Default};
use base::undef::{UndefState};
use atomic::{AtomicUsize};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

struct Inner<T: ?Sized, H = alloc::Heap>
    where H: Allocator,
{
    count: AtomicUsize,
    pool: H::Pool,
    val: T,
}

/// A buffer used when creating a new `Arc`.
pub struct ArcBuf<T, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: OwnedPtr<Inner<T, Heap>>,
}

impl<T, H> ArcBuf<T, H>
    where H: Allocator,
          T: Leak,
{
    /// Stores a value in the buffer, creating a real `Arc`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Arc<T, H> {
        unsafe {
            let data = self.data;
            intrinsics::forget(self);
            ptr::write(&mut (**data).val, val);
            Arc { data: data }
        }
    }
}

unsafe impl<T, H> Send for ArcBuf<T, H> where T: Leak, H: Send+Allocator { }
unsafe impl<T, H> Sync for ArcBuf<T, H> where T: Leak, H: Allocator { }

impl<T, H> Drop for ArcBuf<T, H>
    where H: Allocator,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let mut pool = ptr::read(&(**self.data).pool);
            H::free(&mut pool, *self.data);
        }
    }
}

/// An atomically reference-counted container.
pub struct Arc<T: ?Sized, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: OwnedPtr<Inner<T, Heap>>,
}

impl<T, H> Arc<T, H>
    where H: Allocator,
          H::Pool: Default,
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
    pub fn new() -> Result<ArcBuf<T, H>> {
        unsafe {
            let mut pool = H::Pool::default();
            let data_ptr = try!(H::allocate::<Inner<T, H>>(&mut pool));
            ptr::write(&mut (*data_ptr).pool, pool);
            (*data_ptr).count.store(1);
            Ok(ArcBuf { data: OwnedPtr::new(data_ptr) })
        }
    }
}

impl<T: ?Sized, H> Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    /// Returns a mutable reference to the contained data if this is the only reference.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut **self.data };
        match data.count.load() {
            1 => Some(&mut data.val),
            _ => None,
        }
    }

    /// Adds a new reference, returning an `Arc` that points to the same data.
    pub fn add_ref(&self) -> Arc<T, H> {
        unsafe {
            let data = &mut **self.data;
            data.count.add(1);
            Arc { data: self.data }
        }
    }
}

impl<T: ?Sized, U: ?Sized, H> CoerceUnsized<Arc<U, H>> for Arc<T, H>
    where T: Unsize<U> + Leak,
          U: Leak,
          H: Allocator,
{}

impl<'a, T, H> IntoIterator for &'a Arc<T, H>
    where &'a T: IntoIterator,
          H: Allocator,
          T: Leak,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (**self).into_iter() }
}

unsafe impl<T, H> UndefState for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn num() -> usize { <OwnedPtr<Inner<T, H>> as UndefState>::num() }

    unsafe fn set_undef(val: *mut Arc<T, H>, n: usize) {
        <OwnedPtr<Inner<T, H>> as UndefState>::set_undef(&mut (*val).data, n);
    }

    unsafe fn is_undef(val: *const Arc<T, H>, n: usize) -> bool {
        <OwnedPtr<Inner<T, H>> as UndefState>::is_undef(&(*val).data, n)
    }
}

unsafe impl<T: ?Sized, H> Send for Arc<T, H> where T: Sync+Send+Leak, H:Send+Allocator { }
unsafe impl<T: ?Sized, H> Sync for Arc<T, H> where T: Sync+Leak, H: Allocator { }

impl<T: ?Sized, H> Drop for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let data = &mut **self.data;
            if data.count.sub(1) == 1 {
                if mem::needs_drop::<T>() {
                    ptr::drop(&mut data.val);
                }
                let mut pool = ptr::read(&data.pool);
                H::free(&mut pool, *self.data);
            }
        }
    }
}

impl<T: ?Sized, H> Deref for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&**self.data).val }
    }
}

impl<T: ?Sized, H> Clone for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn clone(&self) -> Arc<T, H> {
        self.add_ref()
    }
}

impl<T: ?Sized, H> MaybeClone for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn maybe_clone(&self) -> Result<Arc<T, H>> {
        Ok(self.add_ref())
    }
}

impl<T: ?Sized, H> Debug for Arc<T, H>
    where T: Debug + Leak,
          H: Allocator,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Arc {{ data: {:?} }}", self.deref())
    }
}
