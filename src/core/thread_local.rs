// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Interrupt, NoSync, Sync};
use ops::{Deref};

/// [hidden]
pub struct __ThreadLocal<T>
    where T: Interrupt
{
    val: T,
    _marker: NoSync,
}

impl<T> __ThreadLocal<T>
    where T: Interrupt
{
    pub const fn new(val: T) -> __ThreadLocal<T> {
        __ThreadLocal {
            val: val,
            _marker: NoSync,
        }
    }
}

impl<T> Deref for __ThreadLocal<T>
    where T: Interrupt
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}

unsafe impl<T> Sync for __ThreadLocal<T> where T: Interrupt { }
