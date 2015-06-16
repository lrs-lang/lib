// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Copy};

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
pub unsafe trait UnusedState {
    /// A copyable representation of an unused bit pattern.
    type Plain: Copy;
    /// The number of available unused states.
    const NUM: usize;

    /// Returns an unused state.
    ///
    /// [argument, n]
    /// The id of the unused state. This value must be below `Self::NUM`, otherwise the
    /// process is aborted.
    fn unused_state(n: usize) -> Self::Plain;
}

unsafe impl UnusedState for bool {
    type Plain = u8;
    const NUM: usize = 256 - 2;

    fn unused_state(n: usize) -> u8 {
        assert!(n < Self::NUM);
        n as u8 + 2
    }
}

unsafe impl UnusedState for char {
    type Plain = u32;
    const NUM: usize = 0xE000 - 0xD800;

    fn unused_state(n: usize) -> u32 {
        assert!(n < Self::NUM);
        n as u32 + 0xE000
    }
}

// NOTE: It is possible for the first page to be mapped but in this case we're already
// fucked because of the null pointer optimization performed by the compiler.

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "arm",
          target_arch = "aarch64"))]
const PAGE_SIZE: usize = 4096;

unsafe impl<'a, T> UnusedState for &'a T {
    type Plain = usize;
    const NUM: usize = PAGE_SIZE;

    fn unused_state(n: usize) -> usize {
        assert!(n < PAGE_SIZE);
        n
    }
}

unsafe impl<'a, T> UnusedState for &'a mut T {
    type Plain = usize;
    const NUM: usize = PAGE_SIZE;

    fn unused_state(n: usize) -> usize {
        assert!(n < PAGE_SIZE);
        n
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
