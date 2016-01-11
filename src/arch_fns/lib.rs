// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_arch_fns"]
#![crate_type = "lib"]
#![feature(asm, lang_items)]
#![no_std]
#![no_builtins]

#[cfg(not(no_libc))] extern crate lrs_libc as libc;
extern crate lrs_cty_base as cty_base;

#[cfg(not(no_libc))] #[path = "libc.rs"] mod imp;
#[cfg(no_libc)] #[path = "no_libc.rs"] pub mod imp;

/// Returns the first occurrence of a byte in a byte slice if any.
///
/// [argument, s]
/// The slice to be searched.
///
/// [argument, c]
/// The byte to be searched for.
///
/// [return_value]
/// Returns the first occurrence of the byte in the slice.
pub fn memchr(s: &[u8], c: u8) -> Option<usize> {
    match unsafe { imp::memchr(s.as_ptr(), c as i32, s.len()) as usize } {
        0 => None,
        n => Some(n - s.as_ptr() as usize),
    }
}

/// Returns the last occurrence of a byte in a byte slice if any.
///
/// [argument, s]
/// The slice to be searched.
///
/// [argument, c]
/// The byte to be searched for.
///
/// [return_value]
/// Returns the last occurrence of the byte in the slice.
pub fn memrchr(s: &[u8], c: u8) -> Option<usize> {
    match unsafe { imp::memrchr(s.as_ptr(), c as i32, s.len()) as usize } {
        0 => None,
        n => Some(n - s.as_ptr() as usize),
    }
}

/// Returns whether the two byte slices are equal.
///
/// [argument, one]
/// The first slice.
///
/// [argument, two]
/// The second slice.
///
/// [return_value]
/// Returns whether `one` and `two` are equal.
pub fn equal(one: &[u8], two: &[u8]) -> bool {
    if one.len() != two.len() {
        return false;
    }
    if one.addr() == two.addr() {
        return true;
    }
    unsafe { imp::memcmp(one.as_ptr(), two.as_ptr(), one.len()) == 0 }
}

/// Returns whether all bytes in a byte slice have a specified value.
///
/// [argument, buf]
/// The slice to be checked.
///
/// [argument, val]
/// The expected value.
///
/// [return_value]
/// Returns whether all bytes in `buf` have the value `val`.
pub fn all_bytes(buf: &[u8], val: u8) -> bool {
    if buf.len() == 0 {
        true
    } else if buf[0] != val {
        false
    } else {
        let len = buf.len();
        equal(&buf[0..len - 1], &buf[1..len])
    }
}

/// Returns the length of a C string.
///
/// [argument, ptr]
/// A pointer to the string.
///
/// [return_value]
/// Returns the length of the string excluding the terminating null byte.
///
/// = Remarks
///
/// If the argument does not point to a null terminated string, the behavior is undefined.
pub unsafe fn strlen(ptr: *const u8) -> usize {
    imp::strlen(ptr)
}

/// Returns whether two strings are equal.
///
/// [argument, a]
/// The first string.
///
/// [argument, b]
/// The second string.
///
/// [return_value]
/// Returns whether both strings are equal.
#[lang = "str_eq"]
pub fn str_equal(a: &str, b: &str) -> bool {
    equal(a.as_bytes(), b.as_bytes())
}

/// Informs the CPU that we're in a spin-wait loop.
///
/// = Remarks
///
/// :icc: link:https://software.intel.com/sites/products/documentation/doclib/iss/2013/compiler/cpp-lin/GUID-3488E3C1-33C3-4444-9D72-CB428DCA3658.htm
///
/// This is currently only implemented for `x86` and `x86_64` processors. From the
/// {icc}[ICC manual]:
///
/// [quote]
/// {
/// The pause intrinsic is used in spin-wait loops with the processors implementing
/// dynamic execution (especially out-of-order execution). In the spin-wait loop, the
/// pause intrinsic improves the speed at which the code detects the release of the lock
/// and provides especially significant performance gain.
///
/// The execution of the next instruction is delayed for an implementation-specific amount
/// of time. The PAUSE instruction does not modify the architectural state. For dynamic
/// scheduling, the PAUSE instruction reduces the penalty of exiting from the spin-loop.
///
/// }
pub fn spin() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    fn spin_int() {
        unsafe { asm!("pause" : : : "memory"); }
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn spin_int() {
        unsafe { core::intrinsics::atomic_fence(); }
    }

    spin_int();
}

/// Copies aligned bytes.
///
/// [argument, dst]
/// The destination to which the bytes will be copied. Aligned to an 16 byte boundary.
///
/// [argument, src]
/// The source frome which the bytes will be copied. Aligned to an 16 byte boundary.
/// Does not overlap with `dst`.
///
/// [argument, len]
/// The number of bytes to copy. A multiple of 16.
pub unsafe fn memcpy_aligned_16_16(dst: *mut d8, src: *const d8, len: usize) {
    let (div, mut rem) = (len / 64, len % 64);
    let mut dst = dst as *mut u64;
    let mut src = src as *const u64;
    while rem > 0 {
        *dst = *src;
        *dst.add(1) = *src.add(1);
        dst = dst.add(2);
        src = src.add(2);
        rem -= 16;
    }
    memcpy_aligned_16_64(dst as *mut _, src as *mut _, div * 64);
}

/// Copies aligned bytes.
///
/// [argument, dst]
/// The destination to which the bytes will be copied. Aligned to an 16 byte boundary.
///
/// [argument, src]
/// The source frome which the bytes will be copied. Aligned to an 16 byte boundary.
/// Does not overlap with `dst`.
///
/// [argument, len]
/// The number of bytes to copy. A multiple of 64.
pub unsafe fn memcpy_aligned_16_64(dst: *mut d8, src: *const d8, len: usize) {
    let mut dst = dst as *mut u64;
    let mut src = src as *const u64;
    let dst_end = dst.add(len / 8);
    while dst != dst_end {
        *dst = *src;
        *dst.add(1) = *src.add(1);
        *dst.add(2) = *src.add(2);
        *dst.add(3) = *src.add(3);
        *dst.add(4) = *src.add(4);
        *dst.add(5) = *src.add(5);
        *dst.add(6) = *src.add(6);
        *dst.add(7) = *src.add(7);
        dst = dst.add(8);
        src = src.add(8);
    }
}
