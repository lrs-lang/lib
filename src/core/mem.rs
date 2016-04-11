// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use ptr::{self};
use marker::{Pod, Copy, Leak, Sized};
use cmp::{self};
use slice::{self};
use option::{Option};
use option::Option::{Some, None};
use data::{d8};

pub use intrinsics::{
    uninit,
};

// TODO: We need a safe version of cast for Pod
pub use intrinsics::transmute as cast;

/// Creates a sequence of zero bytes.
///
/// [return_value]
/// A value that contains a sequence of zero bytes.
pub fn zeroed<T>() -> T
    where T: Pod,
{
    unsafe { intrinsics::init() }
}

/// Returns an unspecified byte sequence of a value.
///
/// [argument, val]
/// The value.
///
/// [return_value]
/// An unspecified byte sequence of the value.
///
/// = Description
///
/// Each byte in the returned slice can be undefined for its type.
pub fn as_data<T: ?Sized>(val: &T) -> &[d8] {
    unsafe { slice::from_ptr(val as *const _ as *const d8, size_of_val(val)) }
}

/// Returns an unspecified mutable byte sequence of a value.
///
/// [argument, val]
/// The value.
///
/// [return_value]
/// An unspecified mutable byte sequence of the value.
///
/// = Description
///
/// Each byte in the returned slice can be undefined for its type.
pub fn as_mut_data<T: ?Sized>(val: &mut T) -> &mut [d8]
    where T: Pod,
{
    unsafe { slice::from_ptr(val as *mut _ as *const d8, size_of_val(val)) }
}

/// Returns whether a buffer is suitable to hold an object of a certain type.
///
/// [argument, buf]
/// The buffer to be checked.
///
/// = Remarks
///
/// The buffer is suitable if it is large enough to hold the type and properly aligned.
pub fn is_suitable_for<T>(buf: &[u8]) -> bool {
    (buf.len() >= size_of::<T>()) && (buf.as_ptr() as usize & (align_of::<T>() - 1) == 0)
}

/// Turns a slice into a reference to a Pod type if it's suitable.
///
/// [argument, buf]
/// The buffer to be turned into a reference.
///
/// [return_value]
/// Returns the created reference.
///
/// = Remarks
///
/// The buffer is suitable under the conditions described in
/// link:lrs::mem::is_suitable_for[is_suitable_for].
pub fn from_bytes<T>(buf: &[u8]) -> Option<&T>
    where T: Pod,
{
    match is_suitable_for::<T>(buf) {
        true => Some(unsafe { &*(buf.as_ptr() as *const T) }),
        _ => None,
    }
}

/// Turns a mutable slice into a mutable reference to a Pod type if it's suitable.
///
/// [argument, buf]
/// The buffer to be turned into a reference.
///
/// [return_value]
/// Returns the created reference.
///
/// = Remarks
///
/// The buffer is suitable under the conditions described in
/// link:lrs::mem::is_suitable_for[is_suitable_for].
pub fn from_mut_bytes<T>(buf: &mut [u8]) -> Option<&mut T>
    where T: Pod,
{
    match is_suitable_for::<T>(buf) {
        true => Some(unsafe { &mut *(buf.as_mut_ptr() as *mut T) }),
        _ => None,
    }
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

/// Destroys an object without running its destructor even if the type does not implement
/// `Leak`.
///
/// [argument, val]
/// The object to be destroyed.
pub unsafe fn unsafe_forget<T>(val: T) {
    intrinsics::forget(val);
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

/// Returns the size of an object.
pub fn size_of_val<T: ?Sized>(v: &T) -> usize {
    unsafe { intrinsics::size_of_val(v) }
}

/// Returns the alignment required for a type.
pub fn align_of<T>() -> usize {
    unsafe { intrinsics::min_align_of::<T>() }
}

/// Returns the alignment required for a type.
pub fn align_of_val<T: ?Sized>(v: &T) -> usize {
    unsafe { intrinsics::min_align_of_val(v) }
}

/// Returns whether a type has a `Drop` implementation.
pub fn needs_drop<T: ?Sized>() -> bool {
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
pub fn align_for<T>(buf: &[u8]) -> &[u8] {
    let align_mask = align_of::<T>() - 1;
    let addr = buf.as_ptr() as usize;
    let diff = ((!addr & align_mask) + 1) & align_mask;
    if diff <= buf.len() {
        &buf[diff..]
    } else {
        &[]
    }
}

/// Left-trims a mutable byte slice so that the first element is aligned.
///
/// [argument, buf]
/// The slice to be trimmed.
///
/// = Remarks
///
/// That is, if the returned slice is not empty, the address of the first element is a
/// multiple of the alignment of `T`.
pub fn align_for_mut<T>(buf: &mut [u8]) -> &mut [u8] {
    let align_mask = align_of::<T>() - 1;
    let addr = buf.as_ptr() as usize;
    let diff = ((!addr & align_mask) + 1) & align_mask;
    if diff <= buf.len() {
        &mut buf[diff..]
    } else {
        &mut []
    }
}

/// Returns the address of an object.
///
/// [argument, obj]
/// The object whose address will be returned.
pub fn addr<T>(obj: &T) -> usize {
    obj as *const T as usize
}

/// Performs a volatile load.
///
/// [argument, src]
/// The source to load from.
///
/// [return_value]
/// Returns the loaded value.
///
/// = See also
///
/// * link:lrs::ptr::volatile_load
pub fn volatile_load<T>(src: &T) -> T {
    unsafe { intrinsics::volatile_load(src) }
}

/// Performs a volatile store.
///
/// [argument, dst]
/// The destination to write to.
///
/// [argument, val]
/// The value to write.
///
/// = See also
///
/// * link:lrs::ptr::volatile_store
pub fn volatile_store<T>(dst: &mut T, val: T) {
    unsafe { intrinsics::volatile_store(dst, val); }
}

/// Calculates the discriminant of an enum variant.
///
/// [argument, v]
/// The variant whose discriminant will be calculated.
///
/// [return_value]
/// Returns the discriminant value cast to an `u64` or `0` if the argument does not
/// have a discriminant value.
///
/// = Remarks
///
/// If the discriminant cannot be represented in an `i64`, the result is unspecified.
/// Note that it is `i64`, not `u64`.
///
/// = Examples
///
/// ----
/// enum T {
///     X,
///     Y,
/// }
///
/// assert!(unsafe { discriminant_value(&T::X) } == 0);
/// ----
pub fn discriminant_value<T>(v: &T) -> u64 {
    unsafe { intrinsics::discriminant_value(v) }
}
