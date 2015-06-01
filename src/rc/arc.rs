// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Deref};
use core::{mem, ptr, intrinsics};
use core::marker::{Leak};
use base::clone::{Clone};
use base::default::{Default};
use atomic::{AtomicUsize};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

struct Inner<T, H>
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
    data: *mut Inner<T, Heap>,
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
            ptr::write(&mut (*data).val, val);
            Arc { data: data }
        }
    }
}

unsafe impl<T, H> Send for ArcBuf<T, H> where H: Send { }
unsafe impl<T, H> Sync for ArcBuf<T, H> { }

impl<T, H> Drop for ArcBuf<T, H>
    where H: Allocator,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let mut pool = ptr::read(&(*self.data).pool);
            H::free(&mut pool, self.data);
        }
    }
}

/// An atomically reference-counted container.
pub struct Arc<T, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: *mut Inner<T, Heap>,
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
            Ok(ArcBuf { data: data_ptr })
        }
    }
}

impl<T, H> Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    /// Returns a mutable reference to the contained data if this is the only reference.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count.load() {
            1 => Some(&mut data.val),
            _ => None,
        }
    }

    /// Adds a new reference, returning an `Arc` that points to the same data.
    pub fn add_ref(&self) -> Arc<T, H> {
        unsafe {
            let data = &mut *self.data;
            data.count.add(1);
            Arc { data: self.data }
        }
    }
}

unsafe impl<T, H> Send for Arc<T, H> where T: Sync+Send, H: Send { }
unsafe impl<T, H> Sync for Arc<T, H> where T: Sync { }

impl<T, H> Drop for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.data;
            if data.count.sub(1) == 1 {
                if mem::needs_drop::<T>() {
                    ptr::drop(&mut data.val);
                }
                let mut pool = ptr::read(&data.pool);
                H::free(&mut pool, self.data);
            }
        }
    }
}

impl<T, H> Deref for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&*self.data).val }
    }
}

impl<T, H> Clone for Arc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn clone(&self) -> Result<Arc<T, H>> {
        Ok(self.add_ref())
    }
}

impl<T, H> Debug for Arc<T, H>
    where T: Debug + Leak,
          H: Allocator,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Arc {{ data: {:?} }}", self.deref())
    }
}
