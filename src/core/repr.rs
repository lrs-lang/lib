// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Copy};
use mem::{self};

/// The representation of `&[T]` or `&mut [T]`.
#[repr(C)]
pub struct Slice<T> {
    /// The pointer to the first element of the slice.
    pub ptr: *const T,
    /// The number of elements in the slice.
    pub len: usize,
}

impl<T> Copy for Slice<T> {}

/// The representation of `&Trait`.
#[repr(C)]
pub struct TraitObject {
    /// The pointer to the data in the slice.
    pub data: *mut u8,
    /// The pointer to the vtable of the slice.
    pub vtable: *mut u8,
}

/// Objects that have an alternative representation.
pub trait Repr<T> {
    /// Returns the alternative representation fo the object.
    fn repr(&self) -> T;
}

impl<T> Repr<Slice<T>> for [T] {
    fn repr(&self) -> Slice<T> { unsafe { mem::copy_as(&self) } }
}

impl Repr<Slice<u8>> for str {
    fn repr(&self) -> Slice<u8> { unsafe { mem::copy_as(&self) } }
}
