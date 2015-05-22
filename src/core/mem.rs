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

/// Returns the in-memory representation of an object.
///
/// [argument, val]
/// The object whose representation is returned.
pub fn as_bytes<T>(val: &T) -> &[u8] {
    unsafe { slice::from_ptr(val as *const _ as *const u8, size_of::<T>()) }
}

/// Returns the mutable in-memory representation of an object.
///
/// [argument, val]
/// The object whose representation is returned.
///
/// = Remarks
///
/// This only accepts `Pod` data because the return value can be used to store arbitrary
/// data in `val`.
pub fn as_mut_bytes<T>(val: &mut T) -> &mut [u8]
    where T: Pod,
{
    unsafe { slice::from_ptr(val as *mut _ as *const u8, size_of::<T>()) }
}

/// Creates an object that has all bytes set to zero.
pub unsafe fn unsafe_zeroed<T>() -> T {
    intrinsics::init()
}

/// Copies an object and casts the result to another type.
///
/// [argument, val]
/// The object to be copied.
///
/// = Remarks
///
/// `T` and `U` can have different sizes but if the size of `U` is larger than `T` and
/// reading from the trailing bytes causes invalid memory access, the behavior is
/// undefined.
pub unsafe fn copy_as<T, U>(src: &T) -> U {
    ptr::read(src as *const T as *const U)
}

/// Destroys an object without running its destructor.
///
/// [argument, val]
/// The object to be destroyed.
pub fn forget<T: Leak>(val: T) {
    unsafe { intrinsics::forget(val); }
}

/// Drops a value.
///
/// [argument, _val]
/// The object to be dropped.
pub fn drop<T>(_val: T) { }

/// Copies bytes from one slice to another.
///
/// [argument, dst]
/// The slice in which the objects will be stored.
///
/// [argument, src]
/// The slice from which the objects will be copied.
///
/// [return_value]
/// Returns the number of objects copied.
///
/// = Remarks
///
/// The number of entries copied is the minimum length of both slices.
pub fn copy<T: Copy>(dst: &mut [T], src: &[T]) -> usize {
    unsafe { unsafe_copy(dst, src) }
}

/// Copies bytes from one slice to another even if the type does not implement `Copy`.
///
/// [argument, dst]
/// The slice in which the objects will be stored.
///
/// [argument, src]
/// The slice from which the objects will be copied.
///
/// [return_value]
/// Returns the number of objects copied.
///
/// = Remarks
///
/// The number of entries copied is the minimum length of both slices.
pub unsafe fn unsafe_copy<T>(dst: &mut [T], src: &[T]) -> usize {
    let min = cmp::min(dst.len(), src.len());
    ptr::memcpy(dst.as_mut_ptr(), src.as_ptr(), min);
    min
}

/// Swaps two objects.
///
/// [argument, one]
/// Object one.
///
/// [argument, two]
/// Object two.
pub fn swap<T>(one: &mut T, two: &mut T) {
    unsafe {
        let tmp: T = copy_as(one);
        ptr::memcpy(one, two, 1);
        ptr::write(two, tmp)
    }
}

/// Replaces an object by another one.
///
/// [argument, dst]
/// The object whose content will be replaced.
///
/// [argument, val]
/// The object that will be stored in `dst`.
///
/// [return_value]
/// Returns the old value in `dst`.
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

/// Returns the alignment required for a type.
pub fn align_of<T>() -> usize {
    unsafe { intrinsics::min_align_of::<T>() }
}

/// Returns whether a type has a `Drop` implementation.
pub fn needs_drop<T>() -> bool {
    unsafe { intrinsics::needs_drop::<T>() }
}

/// Turns a reference into a one-element slice.
///
/// [argument, val]
/// The object that will be the element of the slice.
pub fn as_slice<T>(val: &T) -> &[T] {
    unsafe { slice::from_ptr(val, 1) }
}

/// Turns a mutable reference into a mutable one-element slice.
///
/// [argument, val]
/// The object that will be the element of the slice.
pub fn as_mut_slice<T>(val: &mut T) -> &mut [T] {
    unsafe { slice::from_ptr(val, 1) }
}

/// Left-trims a byte slice so that the first element is aligned.
///
/// [argument, buf]
/// The slice to be trimmed.
///
/// = Remarks
///
/// That is, if the returned slice is not empty, the address of the first element is a
/// multiple of the alignment of `T`.
pub fn align_for<T>(buf: &mut [u8]) -> &mut [u8] {
    let align_mask = align_of::<T>() - 1;
    let addr = buf.as_ptr() as usize;
    let diff = ((!addr & align_mask) + 1) & align_mask;
    if diff <= buf.len() {
        &mut buf[diff..]
    } else {
        &mut []
    }
}
