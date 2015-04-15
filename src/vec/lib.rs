// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_vec"]
// #![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_ty_one as ty_one;
extern crate linux_error as error;
extern crate linux_alloc as alloc;
extern crate linux_fmt as fmt;
extern crate linux_io as io;

extern crate linux_c_stdio as stdio;

use core::prelude::*;
use core::{mem, ptr, cmp, slice};
use core::ops::{Deref, DerefMut};
use core::iter::{IntoIterator};
use fmt::{Debug};
use io::{Write};
use ty_one::result::{Result};
use ty_one::result::Result::{Ok, Err};

mod linux { pub use ::fmt::linux::*; }

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec { ptr: 0 as *mut T, len: 0, cap: 0 }
    }

    pub fn with_capacity(cap: usize) -> Result<Vec<T>> {
        let size = cap * mem::size_of::<T>();
        if size == 0 {
            return Ok(Vec { ptr: 0 as *mut T, len: 0, cap: cap });
        }
        let align = mem::align_of::<T>();
        let ptr = unsafe { alloc::allocate(size, align) as *mut T };
        if ptr.is_null() {
            return Err(error::NoMemory);
        }
        Ok(Vec { ptr: ptr, len: 0, cap: cap })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn reserve(&mut self, n: usize) -> Result {
        if self.cap - self.len >= n {
            return Ok(());
        }
        let to_reserve = cmp::max(n, self.cap / 2 + 1);
        let old_size = self.cap * mem::size_of::<T>();
        let new_size = (self.len + to_reserve) * mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let ptr = unsafe {
            alloc::reallocate(self.ptr as *mut u8, old_size, new_size, align) as *mut T
        };
        if ptr.is_null() {
            Err(error::NoMemory)
        } else {
            self.ptr = ptr;
            self.cap = self.len + to_reserve;
            Ok(())
        }
    }

    pub fn push(&mut self, val: T) {
        if self.cap == self.len {
            self.reserve(1).unwrap();
        }
        unsafe { ptr::write(self.ptr.add(self.len), val); }
        self.len += 1;
    }

    pub fn push_all(&mut self, vals: &[T]) where T: Copy {
        self.reserve(vals.len()).unwrap();
        let tail = unsafe { slice::from_ptr(self.ptr.add(self.len), vals.len()) };
        mem::copy(tail, vals);
        self.len += vals.len();
    }

    pub fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            n => {
                self.len -= 1;
                unsafe { Some(ptr::read(self.ptr.add(n))) }
            },
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            if mem::needs_drop::<T>() {
                for i in 0..self.len() {
                    ptr::read(self.ptr.add(i));
                }
            }
            alloc::free(self.ptr as *mut u8, self.len, self.cap);
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl<T: Debug> Debug for Vec<T> {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}

fn main() {
    use stdio::{Stdout};
    // let mut vec = Vec::new();
    // vec.push("hurr");
    // vec.push("durr");
    println!("{}", "hurr");
}
