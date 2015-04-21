// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_arch_fns"]
#![crate_type = "lib"]
#![feature(plugin, no_std, asm)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
#[prelude_import] use core::prelude::*;

// TODO: Don't use libc.

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

pub unsafe fn strlen(ptr: *const u8) -> usize {
    #[allow(improper_ctypes)]
    extern { fn strlen(s: *const u8) -> usize; }
    strlen(ptr)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn spin() {
    unsafe { asm!("pause" : : : "memory"); }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub fn spin() {
    atomic::fence_seqcst();
}
