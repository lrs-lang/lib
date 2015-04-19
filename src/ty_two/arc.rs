// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::ops::{Deref};
use core::{mem, ptr};
use ty_one::clone::{Clone};
use arch::atomic::{AtomicUsize};
use io::{Write};
use fmt::{Debug};
use {alloc};

struct Inner<T: Send+Sync> {
    count: AtomicUsize,
    val: T,
}

pub struct Arc<T: Send+Sync> {
    data: *mut Inner<T>,
}

impl<T: Send+Sync> Arc<T> {
    pub fn new(val: T) -> Result<Arc<T>, T> {
        unsafe {
            let data_ptr = alloc::allocate::<Inner<T>>();
            if data_ptr.is_null() {
                return Err(val);
            }
            let mut data = &mut *data_ptr;
            data.count.store(1);
            ptr::write(&mut data.val, val);
            Ok(Arc { data: data_ptr })
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count.load_seqcst() {
            1 => Some(&mut data.val),
            _ => None,
        }
    }
}

impl<T: Send+Sync> Drop for Arc<T> {
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.data;
            if data.count.sub_seqcst(1) == 1 {
                if mem::needs_drop::<T>() {
                    ptr::read(&data.val);
                }
                alloc::free(self.data);
            }
        }
    }
}

impl<T: Send+Sync> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&*self.data).val }
    }
}

impl<T: Send+Sync> Clone for Arc<T> {
    fn clone(&self) -> Result<Arc<T>> {
        unsafe {
            let data = &mut *self.data;
            data.count.add_seqcst(1);
            Ok(Arc { data: self.data })
        }
    }
}

impl<T: Send+Sync+Debug> Debug for Arc<T> {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Arc {{ data: {:?} }}", self.deref())
    }
}
