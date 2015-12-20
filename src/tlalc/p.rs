// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ptr::{AliasMutObjPtr};
use core::{mem};

/// An aliasing raw pointer wrapper.
pub struct P<T>(AliasMutObjPtr<T>);

impl<T> Copy for P<T> { }

impl<T> P<T> {
    /// Creates a new `P`.
    ///
    /// [argument, ptr]
    /// The pointer that will be wrapped. Must be a valid pointer.
    pub const unsafe fn new(ptr: *const T) -> P<T> {
        P(AliasMutObjPtr::new(ptr as *mut T))
    }

    pub fn ptr(self) -> *mut T {
        unsafe { mem::cast(self) }
    }

    pub fn to_opt(self) -> Option<P<T>> {
        unsafe { mem::cast(self) }
    }
}

impl<T> Deref for P<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for P<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
