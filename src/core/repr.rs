// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clone::{Clone};
use marker::{Copy};
use mem::{self};

#[repr(C)]
pub struct Slice<T> {
    pub ptr: *const T,
    pub len: usize,
}

impl<T> Copy for Slice<T> {}

impl<T> Clone for Slice<T> {
    fn clone(&self) -> Slice<T> { *self }
}

#[repr(C)]
pub struct TraitObject {
    pub data: *mut u8,
    pub vtable: *mut u8,
}

pub unsafe trait Repr<T> {
    fn repr(&self) -> T;
}

unsafe impl<T> Repr<Slice<T>> for [T] {
    fn repr(&self) -> Slice<T> { unsafe { mem::copy_as(&self) } }
}

unsafe impl Repr<Slice<u8>> for str {
    fn repr(&self) -> Slice<u8> { unsafe { mem::copy_as(&self) } }
}
