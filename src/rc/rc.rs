// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Deref};
use core::marker::{Leak};
use core::{mem, ptr, intrinsics};
use base::clone::{Clone};
use base::default::{Default};
use base::unused::{UnusedState};
use cell::copy_cell::{CopyCell};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

struct Inner<T, H>
    where H: Allocator,
{
    count: CopyCell<usize>,
    pool: H::Pool,
    val: T,
}

/// A buffer used when creating a new `Rc`.
pub struct RcBuf<T, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: *mut Inner<T, Heap>,
}

impl<T, H> RcBuf<T, H>
    where H: Allocator,
          T: Leak,
{
    /// Stores a value in the buffer, creating a real `Rc`.
    ///
    /// [argument, val]
    /// The value to be stored in the buffer.
    pub fn set(self, val: T) -> Rc<T, H> {
        unsafe {
            let data = self.data;
            intrinsics::forget(self);
            ptr::write(&mut (*data).val, val);
            Rc { data: data }
        }
    }
}

unsafe impl<T, H> Send for RcBuf<T, H> where H: Send { }
unsafe impl<T, H> Sync for RcBuf<T, H> { }

impl<T, H> Drop for RcBuf<T, H>
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

/// A single-threaded reference-counted container.
pub struct Rc<T, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: *mut Inner<T, Heap>,
}

impl<T, H> Rc<T, H>
    where H: Allocator,
          H::Pool: Default,
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
    pub fn new() -> Result<RcBuf<T, H>> {
        unsafe {
            let mut pool = H::Pool::default();
            let data_ptr = try!(H::allocate::<Inner<T, H>>(&mut pool));
            ptr::write(&mut (*data_ptr).pool, pool);
            (*data_ptr).count.set(1);
            Ok(RcBuf { data: data_ptr })
        }
    }
}

impl<T, H> Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    /// Returns a mutable reference to the contained data if this is the only reference.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count.get() {
            1 => Some(&mut data.val),
            _ => None,
        }
    }

    /// Adds a new reference, returning an `Rc` that points to the same data.
    pub fn add_ref(&self) -> Rc<T, H> {
        unsafe {
            let data = &mut *self.data;
            data.count.set(data.count.get() + 1);
            Rc { data: self.data }
        }
    }
}

unsafe impl<T, H> UnusedState for Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    type Plain = [usize; 2];
    const NUM: usize = <&'static u8 as UnusedState>::NUM;

    fn unused_state(n: usize) -> [usize; 2] {
        let unused_ptr = <&'static u8 as UnusedState>::unused_state(n);

        // The compiler is terminally broken and won't let us use use <T, H> here. It will
        // then complain that we have to use `[usize; 3]` and compile fine with that.
        // After monomorphization it promptly ICEs.

        unsafe { mem::cast(Rc { data: unused_ptr as *mut Inner<T, alloc::Heap> }) }
    }
}

impl<T, H> !Send for Rc<T, H> { }

impl<T, H> Drop for Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.data;
            let count = data.count.get();
            data.count.set(count - 1);
            if count == 1 {
                if mem::needs_drop::<T>() {
                    ptr::drop(&mut data.val);
                }
                let mut pool = ptr::read(&data.pool);
                H::free(&mut pool, self.data);
            }
        }
    }
}

impl<T, H> Deref for Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&*self.data).val }
    }
}

impl<T, H> Clone for Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    fn clone(&self) -> Result<Rc<T, H>> {
        Ok(self.add_ref())
    }
}

impl<T, H> Debug for Rc<T, H>
    where T: Debug + Leak,
          H: Allocator,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Rc {{ data: {:?} }}", self.deref())
    }
}
