// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sized};
use core::{mem};

/// Types that can never match certain bit patterns.
///
/// = Remarks
///
/// This trait is for types such that valid instances of it can never match certain bit
/// patterns. For example, since the capacity of vectors is bounded by `isize::max()`, all
/// bit patterns that have the highest bit in the capacity field set cannot be real
/// instances of a vector.
///
/// This can be used to efficiently store additional state, for example, a hash table can
/// use this to store the status of unused buckets inline, reducing the memory usage per
/// bucket by up to eight bytes.
///
/// == Invariants
///
/// An implementation of this trait must have the following properties:
///
/// **
/// {
/// For all `n != m < Self::NUM`:
///
/// ----
/// mem::as_bytes(&Self::unused_state(n)) != mem::as_bytes(&Self::unused_state(m))
/// ----
/// }
/// * `mem::size_of::<Self::Plain>() == mem::size_of::<Self>()`
/// * If `Self::unused_state` is called with an argument `>= Self::NUM`, the process is
///   aborted.
pub unsafe trait UndefState: Sized {
    /// The number of available unused states.
    fn num() -> usize;

    /// Returns an unused state.
    ///
    /// [argument, n]
    /// The id of the unused state. This value must be below `Self::NUM`, otherwise the
    /// behavior is undefined.
    unsafe fn set_undef(val: *mut Self, n: usize);

    unsafe fn is_undef(val: *const Self, n: usize) -> bool;
}

unsafe impl UndefState for bool {
    fn num() -> usize { 256 - 2 }

    unsafe fn set_undef(val: *mut bool, n: usize) {
        assert!(n < Self::num());
        assert!(mem::size_of::<bool>() == 1);
        *(val as *mut u8) = n as u8 + 2;
    }

    unsafe fn is_undef(val: *const bool, n: usize) -> bool {
        *(val as *const u8) as usize == n + 2
    }
}

unsafe impl UndefState for char {
    fn num() -> usize { 0xE000 - 0xD800 }

    unsafe fn set_undef(val: *mut char, n: usize) {
        assert!(n < Self::num());
        assert!(mem::size_of::<char>() == 4);
        *(val as *mut u32) = n as u32 + 0xE000;
    }

    unsafe fn is_undef(val: *const char, n: usize) -> bool {
        *(val as *const u32) == n as u32 + 0xE000
    }
}

// NOTE: It is possible for the first page to be mapped but in this case we're already
// fucked because of the null pointer optimization performed by the compiler.

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "arm",
          target_arch = "aarch64"))]
const PAGE_SIZE: usize = 4096;

unsafe impl<'a, T> UndefState for &'a T {
    fn num() -> usize { PAGE_SIZE }

    unsafe fn set_undef(val: *mut &'a T, n: usize) {
        assert!(n < Self::num());
        *(val as *mut usize) = n;
    }

    unsafe fn is_undef(val: *const &'a T, n: usize) -> bool {
        *(val as *const usize) == n
    }
}

unsafe impl<'a, T> UndefState for &'a mut T {
    fn num() -> usize { PAGE_SIZE }

    unsafe fn set_undef(val: *mut &'a mut T, n: usize) {
        assert!(n < Self::num());
        *(val as *mut usize) = n;
    }

    unsafe fn is_undef(val: *const &'a mut T, n: usize) -> bool {
        *(val as *const usize) == n
    }
}

// FIXME: The following implementations conflict with the implementations above for some
// reason. The compiler is being retarded again. Fork and fix.

// unsafe impl<'a, T> UnusedState for &'a [T] {
//     type Plain = [usize; 2];
//     const NUM: usize = (!0 >> 1) + 1;
// 
//     fn unused_state(n: usize) -> [usize; 2] {
//         assert!(n <= !0 >> 1);
//         unsafe { mem::cast(slice::from_ptr(0 as *const T, !0 - n)) }
//     }
// }
//
// unsafe impl<'a, T> UnusedState for &'a mut [T] {
//     type Plain = [usize; 2];
//     const NUM: usize = (!0 >> 1) + 1;
// 
//     fn unused_state(n: usize) -> [usize; 2] {
//         assert!(n <= !0 >> 1);
//         unsafe { mem::cast(slice::from_ptr(0 as *const T, !0 - n)) }
//     }
// }
//
// unsafe impl<'a> UnusedState for &'a str {
//     type Plain = [usize; 2];
//     const NUM: usize = (!0 >> 1) + 1;
// 
//     fn unused_state(n: usize) -> [usize; 2] {
//         assert!(n <= !0 >> 1);
//         unsafe { mem::cast(slice::from_ptr(0 as *const u8, !0 - n)) }
//     }
// }
