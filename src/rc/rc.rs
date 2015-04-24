// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Deref};
use core::marker::{Leak};
use core::{mem, ptr};
use base::clone::{Clone};
use cell::copy_cell::{CopyCell};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

struct Inner<T> {
    count: CopyCell<usize>,
    val: T,
}

pub struct Rc<T, Heap = alloc::Heap>
    where Heap: Allocator,
          T: Leak,
{
    data: *mut Inner<T>,
    _marker: PhantomData<Heap>,
}

impl<T, H> Rc<T, H>
    where H: Allocator,
          T: Leak,
{
    pub fn new(val: T) -> Result<Rc<T, H>, T> {
        unsafe {
            let data_ptr = match H::allocate::<Inner<T>>() {
                Ok(p) => p,
                _ => return Err(val),
            };
            let mut data = &mut *data_ptr;
            data.count.set(1);
            ptr::write(&mut data.val, val);
            Ok(Rc { data: data_ptr, _marker: PhantomData, })
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count.get() {
            1 => Some(&mut data.val),
            _ => None,
        }
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
                    ptr::read(&data.val);
                }
                H::free(self.data);
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
        unsafe {
            let data = &mut *self.data;
            data.count.set(data.count.get() + 1);
            Ok(Rc { data: self.data, _marker: PhantomData })
        }
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
