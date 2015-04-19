// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::ops::{Deref};
use core::{mem, ptr};
use ty_one::clone::{Clone};
use ty_one::copy_cell::{CopyCell};
use io::{Write};
use fmt::{Debug};
use alloc::{allocate, free};

struct Inner<T> {
    count: CopyCell<usize>,
    val: T,
}

pub struct Rc<T> {
    data: *mut Inner<T>,
}

impl<T> Rc<T> {
    pub fn new(val: T) -> Result<Rc<T>, T> {
        unsafe {
            let data_ptr = allocate::<Inner<T>>();
            if data_ptr.is_null() {
                return Err(val);
            }
            let mut data = &mut *data_ptr;
            data.count.set(1);
            ptr::write(&mut data.val, val);
            Ok(Rc { data: data_ptr })
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

impl<T> !Send for Rc<T> { }

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let data = &mut *self.data;
            let count = data.count.get();
            data.count.set(count - 1);
            if count == 1 {
                if mem::needs_drop::<T>() {
                    ptr::read(&data.val);
                }
                free(self.data);
            }
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(&*self.data).val }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Result<Rc<T>> {
        unsafe {
            let data = &mut *self.data;
            data.count.set(data.count.get() + 1);
            Ok(Rc { data: self.data })
        }
    }
}

impl<T: Debug> Debug for Rc<T> {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Rc {{ data: {:?} }}", self.deref())
    }
}
