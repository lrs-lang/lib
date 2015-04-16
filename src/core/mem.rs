// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use intrinsics::{self};
use ptr::{self};
use marker::{Copy};
use cmp::{self};

pub use intrinsics::{
    uninit,
};

pub use intrinsics::transmute as cast;

pub unsafe fn zeroed<T>() -> T { intrinsics::init() }

pub unsafe fn copy_as<T, U>(src: &T) -> U {
    ptr::read(src as *const T as *const U)
}

pub fn forget<T>(val: T) {
    unsafe { intrinsics::forget(val); }
}

pub fn drop<T>(_: T) { }

pub fn copy<T: Copy>(dst: &mut [T], src: &[T]) -> usize {
    unsafe { unsafe_copy(dst, src) }
}

pub unsafe fn unsafe_copy<T>(dst: &mut [T], src: &[T]) -> usize {
    let min = cmp::min(dst.len(), src.len());
    ptr::memcpy(dst.as_mut_ptr(), src.as_ptr(), min);
    min
}

pub fn swap<T>(one: &mut T, two: &mut T) {
    unsafe {
        let tmp: T = copy_as(one);
        ptr::memcpy(one, two, 1);
        ptr::write(two, tmp)
    }
}

pub fn replace<T>(dst: &mut T, val: T) -> T {
    unsafe {
        let res: T = copy_as(dst);
        ptr::write(dst, val);
        res
    }
}

pub fn size_of<T>() -> usize {
    unsafe { intrinsics::size_of::<T>() }
}

pub fn align_of<T>() -> usize {
    unsafe { intrinsics::min_align_of::<T>() }
}

pub fn needs_drop<T>() -> bool {
    unsafe { intrinsics::needs_drop::<T>() }
}
