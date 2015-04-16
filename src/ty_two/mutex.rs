// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem, ptr};
use core::ops::{Deref, DerefMut};
use fmt::{Debug};
use io::{Write};
use lock::{self, Lock, LockGuard};
use {alloc};

struct Inner<T: Send> {
    lock: Lock,
    val: T,
}

pub struct Mutex<T: Send> {
    data: *mut Inner<T>,
}

impl<T: Send> Mutex<T> {
    pub fn new(val: T) -> Result<Mutex<T>, T> {
        unsafe {
            let size = mem::size_of::<Inner<T>>();
            let align = mem::align_of::<Inner<T>>();
            let data_ptr = alloc::allocate(size, align) as *mut Inner<T>;
            if data_ptr.is_null() {
                return Err(val);
            }
            let mut data = &mut *data_ptr;
            data.lock = lock::INIT;
            ptr::write(&mut data.val, val);
            Ok(Mutex { data: data_ptr })
        }
    }

    fn guard<'a>(&'a self, guard: LockGuard) -> MutexGuard<'a, T> {
        MutexGuard {
            _guard: guard,
            val: unsafe { &mut (&mut *self.data).val },
            _marker: PhantomData,
        }
    }

    unsafe fn get_lock(&self) -> &'static Lock {
        mem::cast(&(&*self.data).lock)
    }

    pub fn try_lock<'a>(&'a self) -> Option<MutexGuard<'a, T>> {
        let lock = unsafe { self.get_lock() };
        lock.try_lock().map(|g| self.guard(g))
    }

    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        let lock = unsafe { self.get_lock() };
        self.guard(lock.lock())
    }
}

pub struct MutexGuard<'a, T: Send> {
    _guard: LockGuard,
    val: *mut T,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T: Send> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.val }
    }
}

impl<'a, T: Send> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.val }
    }
}

impl<'a, T: Send+Debug> Debug for MutexGuard<'a, T> {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        self.deref().fmt(w)
    }
}
