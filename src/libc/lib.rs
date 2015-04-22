// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_libc"]
#![crate_type = "lib"]
#![feature(plugin, no_std, lang_items)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;

#[allow(improper_ctypes)]
extern {
    pub static mut environ: *const *const u8;

    pub fn memchr(s: *const u8, c: i32, n: usize) -> *const u8;
    pub fn memrchr(s: *const u8, c: i32, n: usize) -> *const u8;
    pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    pub fn fork() -> i32;
    pub fn __errno_location() -> *mut i32;
    pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub fn strlen(s: *const u8) -> usize;
}
