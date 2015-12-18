// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sized};
use core::{mem};
use core::repr::{Slice};
use core::ptr::{self};

/// Types that are not valid when they contain certain bit patterns.
///
/// = Remarks
///
/// This is the opposite of link:lrs::marker::Pod[Pod]. That is, every type is either
/// `Pod` or `UndefState`. (But note that not all types need to implement either of those
/// types.)
///
/// A type that implements this trait has a constant number of so-called undefined states.
///
/// = Examples
///
/// Objects of type `char` cannot contain surrogate code points. Since `char` is just a
/// wrapper of `u32`, the following implementation is valid and gives us `2048` undefined
/// states:
///
/// ----
/// unsafe impl UndefState for char {
///     fn num() -> usize { 0xE000 - 0xD800 }
///
///     unsafe fn set_undef(val: *mut char, n: usize) {
///         assert!(n < Self::num());
///         *(val as *mut u32) = n as u32 + 0xE000;
///     }
///
///     unsafe fn is_undef(val: *const char, n: usize) -> bool {
///         assert!(n < Self::num());
///         *(val as *const u32) == n as u32 + 0xE000
///     }
/// }
/// ----
pub unsafe trait UndefState: Sized {
    /// The number of available undefined states.
    ///
    /// = Remarks
    ///
    /// This function must be constant.
    fn num() -> usize;

    /// Creates an undefined state.
    ///
    /// [argument, val]
    /// A storage location suitable to hold an object of type `Self`.
    ///
    /// [argument, n]
    /// The id of the undefined state.
    ///
    /// = Remarks
    ///
    /// This function behaves as follows:
    ///
    /// * If `n` is greater than or equal to `Self::num()`, the process is aborted.
    /// * The implementation does not inspect the object pointed to by `val`.
    /// * After the function returns, the storage location does not contain an object of
    ///   type `Self`.
    /// * After the function returns, the storage location is in the undefined state `n`.
    unsafe fn set_undef(val: *mut Self, n: usize);

    /// Checks if a storage location is in an undefined state.
    ///
    /// [argument, val]
    /// A storage location suitable to hold an object of type `Self`.
    ///
    /// [argument, n]
    /// The id of the undefined state.
    ///
    /// = Remarks
    ///
    /// This function behaves as follows:
    ///
    /// * If `n` is greater than or equal to `Self::num()`, the process is aborted.
    /// * If the storage location contains an object of type `Self`, `false` is returned.
    /// * If the storage location is in the undefined state `m != n`, `false` is returned.
    /// * If the storage location is in the undefined state `n`, `true` is returned.
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
        assert!(n < Self::num());
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
        assert!(n < Self::num());
        *(val as *const u32) == n as u32 + 0xE000
    }
}

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
        assert!(n < Self::num());
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
        assert!(n < Self::num());
        *(val as *const usize) == n
    }
}

macro_rules! ptr {
    ($name:ident) => {
        unsafe impl<T> UndefState for ptr::$name<T> {
            fn num() -> usize { PAGE_SIZE }

            unsafe fn set_undef(val: *mut Self, n: usize) {
                assert!(n < Self::num());
                *val = Self::new(n as *mut T);
            }

            unsafe fn is_undef(val: *const Self, n: usize) -> bool {
                assert!(n < Self::num());
                (*val).get() as usize == n
            }
        }
    }
}

ptr!(NoAliasMemPtr);
ptr!(NoAliasMutObjPtr);
ptr!(NoAliasObjPtr);
ptr!(AliasMemPtr);
ptr!(AliasMutObjPtr);
ptr!(AliasObjPtr);

unsafe impl<'a, T> UndefState for &'a [T] {
    fn num() -> usize { PAGE_SIZE }

    unsafe fn set_undef(val: *mut &'a [T], n: usize) {
        assert!(n < Self::num());
        (*(val as *mut Slice<T>)).ptr = n as *mut T;
    }

    unsafe fn is_undef(val: *const &'a [T], n: usize) -> bool {
        assert!(n < Self::num());
        (*(val as *mut Slice<T>)).ptr == n as *mut T
    }
}

unsafe impl<'a, T> UndefState for &'a mut [T] {
    fn num() -> usize { PAGE_SIZE }

    unsafe fn set_undef(val: *mut &'a mut [T], n: usize) {
        assert!(n < Self::num());
        (*(val as *mut Slice<T>)).ptr = n as *mut T;
    }

    unsafe fn is_undef(val: *const &'a mut [T], n: usize) -> bool {
        assert!(n < Self::num());
        (*(val as *mut Slice<T>)).ptr == n as *mut T
    }
}

unsafe impl<'a> UndefState for &'a str {
    fn num() -> usize { PAGE_SIZE }

    unsafe fn set_undef(val: *mut &'a str, n: usize) {
        assert!(n < Self::num());
        (*(val as *mut Slice<u8>)).ptr = n as *mut u8;
    }

    unsafe fn is_undef(val: *const &'a str, n: usize) -> bool {
        assert!(n < Self::num());
        (*(val as *mut Slice<u8>)).ptr == n as *mut u8
    }
}
