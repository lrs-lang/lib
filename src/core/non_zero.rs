// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Copy, Unsize, Sized};
use ops::{CoerceUnsized};

#[lang = "non_zero"]
pub struct NonZero<T>(T);

impl<T: Copy> Copy for NonZero<T> { }

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonZero<*const U>> for NonZero<*const T>
    where T: Unsize<U>,
{}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonZero<*mut U>> for NonZero<*mut T>
    where T: Unsize<U>,
{}

impl<T> NonZero<T> {
    pub const unsafe fn new(val: T) -> NonZero<T> {
        NonZero(val)
    }

    pub fn get(&self) -> T
        where T: Copy
    {
        self.0
    }

    pub unsafe fn set(&mut self, val: T) {
        self.0 = val
    }

    pub const fn into(self) -> T {
        self.0
    }
}
