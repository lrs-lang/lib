// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_arch_fns"]
#![crate_type = "lib"]
#![feature(plugin, no_std, asm, lang_items)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
#[prelude_import] use core::prelude::*;

// TODO: Don't use libc.

/// Returns the first occurrence of a byte in a slice if any.
pub fn memchr(s: &[u8], c: u8) -> Option<usize> {
    #[allow(improper_ctypes)]
    extern {
        fn memchr(s: *const u8, c: i32, n: usize) -> *const u8;
    }
    match unsafe { memchr(s.as_ptr(), c as i32, s.len()) as usize } {
        0 => None,
        n => Some(n - s.as_ptr() as usize),
    }
}

/// Like `memchr` but searches from the end.
pub fn memrchr(s: &[u8], c: u8) -> Option<usize> {
    #[allow(improper_ctypes)]
    extern {
        fn memrchr(s: *const u8, c: i32, n: usize) -> *const u8;
    }
    match unsafe { memrchr(s.as_ptr(), c as i32, s.len()) as usize } {
        0 => None,
        n => Some(n - s.as_ptr() as usize),
    }
}

/// Returns whether the two slices are equal.
pub fn equal(one: &[u8], two: &[u8]) -> bool {
    #[allow(improper_ctypes)]
    extern {
        fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    }
    if one.len() != two.len() {
        return false;
    }
    unsafe { memcmp(one.as_ptr(), two.as_ptr(), one.len()) == 0 }
}

/// Returns whether all bytes in the slice have the specified value.
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

/// Returns the length of the null-terminated string pointed to by `ptr`.
pub unsafe fn strlen(ptr: *const u8) -> usize {
    #[allow(improper_ctypes)]
    extern { fn strlen(s: *const u8) -> usize; }
    strlen(ptr)
}

#[lang = "str_eq"]
pub fn str_equal(a: &str, b: &str) -> bool {
    equal(a.as_bytes(), b.as_bytes())
}

/// Informs the CPU that the execution is blocked by another thread.
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn spin() {
    unsafe { asm!("pause" : : : "memory"); }
}

/// Informs the CPU that the execution is blocked by another thread.
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub fn spin() {
    atomic::fence_seqcst();
}
