// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Deref};
use core::{mem, ptr};
use base::clone::{Clone};
use atomic::{AtomicUsize};
use fmt::{Debug, Write};
use alloc::{self, Allocator};

struct Inner<T> {
    count: AtomicUsize,
    val: T,
}

pub struct Arc<T, Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: *mut Inner<T>,
    _marker: PhantomData<Heap>,
}

impl<T, H> Arc<T, H>
    where H: Allocator,
{
    pub fn new(val: T) -> Result<Arc<T>, T> {
        unsafe {
            let data_ptr = match H::allocate::<Inner<T>>() {
                Ok(p) => p,
                _ => return Err(val),
            };
            let mut data = &mut *data_ptr;
            data.count.store(1);
            ptr::write(&mut data.val, val);
            Ok(Arc { data: data_ptr, _marker: PhantomData })
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count.load() {
            1 => Some(&mut data.val),
            _ => None,
        }
    }
}

impl<T, H> Drop for Arc<T, H>
    where H: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.data;
            if data.count.sub(1) == 1 {
                if mem::needs_drop::<T>() {
                    ptr::read(&data.val);
                }
                H::free(self.data);
            }
        }
    }
}

impl<T, H> Deref for Arc<T, H>
    where H: Allocator,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&*self.data).val }
    }
}

impl<T, H> Clone for Arc<T, H>
    where H: Allocator,
{
    fn clone(&self) -> Result<Arc<T, H>> {
        unsafe {
            let data = &mut *self.data;
            data.count.add(1);
            Ok(Arc { data: self.data, _marker: PhantomData })
        }
    }
}

impl<T, H> Debug for Arc<T, H>
    where T: Debug,
          H: Allocator,
{
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Arc {{ data: {:?} }}", self.deref())
    }
}
