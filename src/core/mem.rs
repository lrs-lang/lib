// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use ptr::{self};
use marker::{Pod, Copy, Leak};
use cmp::{self};
use slice::{self};

pub use intrinsics::{
    uninit,
};

// TODO: We need a safe version of cast for Pod
pub use intrinsics::transmute as cast;

/// Creates an object that has all bytes set to zero.
pub fn zeroed<T>() -> T
    where T: Pod,
{
    unsafe { intrinsics::init() }
}

/// Returns the in-memory representation of `val`.
pub fn as_bytes<T>(val: &T) -> &[u8] {
    unsafe { slice::from_ptr(val as *const _ as *const u8, size_of::<T>()) }
}

/// Like `as_bytes`.
///
/// This only accepts `Pod` data because the return value can be used to store arbitrary
/// data in `val`.
pub fn as_mut_bytes<T>(val: &mut T) -> &mut [u8]
    where T: Pod,
{
    unsafe { slice::from_ptr(val as *mut _ as *const u8, size_of::<T>()) }
}

/// Like `zeroed` but works for arbitrary types.
pub unsafe fn unsafe_zeroed<T>() -> T {
    intrinsics::init()
}

/// Copies an object and casts the result to another type.
///
/// `T` and `U` can have different sizes but if the size of `U` is larger than `T` and
/// reading from the trailing bytes causes invalid memory access the behavior is
/// undefined.
pub unsafe fn copy_as<T, U>(src: &T) -> U {
    ptr::read(src as *const T as *const U)
}

/// Destroys an object without running its destructor.
pub fn forget<T: Leak>(val: T) {
    unsafe { intrinsics::forget(val); }
}

/// Drops a value.
pub fn drop<T>(_: T) { }

/// Copies bytes from `src` to `dst`.
///
/// The number of entries copied is the minimum length of both slices. Returns the number
/// of entries copied.
pub fn copy<T: Copy>(dst: &mut [T], src: &[T]) -> usize {
    unsafe { unsafe_copy(dst, src) }
}

/// Like `copy` but also works for `!Copy`.
pub unsafe fn unsafe_copy<T>(dst: &mut [T], src: &[T]) -> usize {
    let min = cmp::min(dst.len(), src.len());
    ptr::memcpy(dst.as_mut_ptr(), src.as_ptr(), min);
    min
}

/// Swaps two objects.
pub fn swap<T>(one: &mut T, two: &mut T) {
    unsafe {
        let tmp: T = copy_as(one);
        ptr::memcpy(one, two, 1);
        ptr::write(two, tmp)
    }
}

/// Replaces the object in `dst` by `val` and returns the old object.
pub fn replace<T>(dst: &mut T, val: T) -> T {
    unsafe {
        let res: T = copy_as(dst);
        ptr::write(dst, val);
        res
    }
}

/// Returns the size of an object.
pub fn size_of<T>() -> usize {
    unsafe { intrinsics::size_of::<T>() }
}

/// Returns the alignment required by the architecture for this object.
pub fn align_of<T>() -> usize {
    unsafe { intrinsics::min_align_of::<T>() }
}

/// Returns whether this object has a `Drop` implementation.
pub fn needs_drop<T>() -> bool {
    unsafe { intrinsics::needs_drop::<T>() }
}
