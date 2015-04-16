// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::ops::{Deref};
use core::{mem, ptr};
use core::clone::{Clone};
use io::{Write};
use fmt::{Debug};
use {alloc};

struct Inner<T> {
    count: usize,
    val: T,
}

pub struct Rc<T> {
    data: *mut Inner<T>,
}

impl<T> Rc<T> {
    pub fn new(val: T) -> Result<Rc<T>, T> {
        unsafe {
            let size = mem::size_of::<Inner<T>>();
            let align = mem::align_of::<Inner<T>>();
            let data_ptr = alloc::allocate(size, align) as *mut Inner<T>;
            if data_ptr.is_null() {
                return Err(val);
            }
            let mut data = &mut *data_ptr;
            data.count = 1;
            ptr::write(&mut data.val, val);
            Ok(Rc { data: data_ptr })
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        let data = unsafe { &mut *self.data };
        match data.count {
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
            data.count -= 1;
            if data.count == 0 {
                if mem::needs_drop::<T>() {
                    ptr::read(&data.val);
                }
                let size = mem::size_of::<Inner<T>>();
                let align = mem::align_of::<Inner<T>>();
                alloc::free(self.data as *mut u8, size, align);
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
    fn clone(&self) -> Rc<T> {
        unsafe {
            let data = &mut *self.data;
            data.count += 1;
            Rc { data: self.data }
        }
    }
}

impl<T: Debug> Debug for Rc<T> {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Rc {{ data: {:?} }}", self.deref())
    }
}
