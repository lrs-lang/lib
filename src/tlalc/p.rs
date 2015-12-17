// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ptr::{NonZeroPtr};
use core::{mem};

pub struct P<T>(pub NonZeroPtr<T>);

impl<T> Copy for P<T> { }

impl<T> P<T> {
    pub const unsafe fn new(ptr: *const T) -> P<T> {
        P(NonZeroPtr::new(ptr))
    }

    pub const unsafe fn zero() -> P<T> {
        P(NonZeroPtr::new(0 as *const T))
    }

    pub fn ptr(self) -> *mut T {
        unsafe { mem::cast(self) }
    }

    pub fn to_opt(self) -> POpt<T> {
        unsafe { mem::cast(self) }
    }
}

impl<T> Deref for P<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &**self.0 }
    }
}

impl<T> DerefMut for P<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *(*self.0 as *mut T) }
    }
}

pub struct POpt<T>(pub Option<P<T>>);

impl<T> Copy for POpt<T> { }

impl<T> POpt<T> {
    pub unsafe fn some(ptr: *const T) -> POpt<T> {
        mem::cast(ptr)
    }

    pub const fn none() -> POpt<T> {
        POpt(None)
    }
}

impl<T> Deref for POpt<T> {
    type Target = Option<P<T>>;
    fn deref(&self) -> &Option<P<T>> {
        unsafe { mem::cast(self) }
    }
}

impl<T> DerefMut for POpt<T> {
    fn deref_mut(&mut self) -> &mut Option<P<T>> {
        unsafe { mem::cast(self) }
    }
}
