// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use cty_base::types::{c_int};

#[no_mangle]
pub unsafe extern fn memchr(mut s: *const u8, c: c_int, n: usize) -> *const u8 {
    let end = s.add(n);
    let c = c as u8;
    while s != end {
        if *s == c {
            return s;
        }
        s = s.add(1);
    }
    0 as *const u8
}

#[no_mangle]
pub unsafe extern fn memrchr(mut s: *const u8, c: c_int, n: usize) -> *const u8 {
    s = s.sub(1);
    let mut end = s.add(n);
    let c = c as u8;
    while s != end {
        if *end == c {
            return end;
        }
        end = end.sub(1);
    }
    0 as *const u8
}

#[no_mangle]
pub unsafe extern fn memcmp(mut s1: *const u8, mut s2: *const u8, n: usize) -> c_int {
    let end = s1.add(n);
    while s1 != end {
        if *s1 == *s2 {
            s1 = s1.add(1);
            s2 = s2.add(1);
            continue;
        }
        if *s1 < *s2 {
            return -1;
        }
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern fn memcpy(mut dst: *mut u8, mut src: *const u8,
                            n: usize) -> *const u8 {
    dst = dst.sub(1);
    src = src.sub(1);
    let mut dst_end = dst.add(n);
    let mut src_end = src.add(n);
    while dst != dst_end {
        *dst_end = *src_end;
        dst_end = dst_end.sub(1);
        src_end = src_end.sub(1);
    }
    dst.add(1)
}

#[no_mangle]
pub unsafe extern fn memmove(mut dst: *mut u8, mut src: *const u8,
                             n: usize) -> *const u8 {
    if src as usize <= dst as usize {
        // memcpy copies from the tail
        return memcpy(dst, src, n);
    }

    let dst_end = dst.add(n);
    while dst != dst_end {
        *dst = *src;
        dst = dst.add(1);
        src = src.add(1);
    }
    dst.sub(n)
}

#[no_mangle]
pub unsafe extern fn strlen(mut s: *const u8) -> usize {
    let mut num = 0;
    while *s != 0 {
        num += 1;
        s = s.add(1);
    }
    num
}

#[no_mangle]
pub unsafe extern fn memset(mut s: *mut u8, c: c_int, n: usize) -> *mut u8 {
    let end = s.add(n);
    let c = c as u8;
    while s != end {
        *s = c;
        s = s.add(1);
    }
    end.sub(n)
}

#[cfg(target_arch = "arm")]
pub mod arch {
    use cty_base::types::{c_int};

    macro_rules! memclr {
        ($name:ident) => {
            #[no_mangle]
            pub unsafe extern fn $name(dst: *mut u8, n: usize) {
                super::memset(dst, 0, n);
            }
        }
    }
    memclr!(__aeabi_memclr);
    memclr!(__aeabi_memclr4);
    memclr!(__aeabi_memclr8);


    macro_rules! memcpy {
        ($name:ident) => {
            #[no_mangle]
            pub unsafe extern fn $name(dst: *mut u8, src: *const u8, n: usize) {
                super::memcpy(dst, src, n);
            }
        }
    }
    memcpy!(__aeabi_memcpy);
    memcpy!(__aeabi_memcpy4);
    memcpy!(__aeabi_memcpy8);


    macro_rules! memmove {
        ($name:ident) => {
            #[no_mangle]
            pub unsafe extern fn $name(dst: *mut u8, src: *const u8, n: usize) {
                super::memmove(dst, src, n);
            }
        }
    }
    memmove!(__aeabi_memmove);
    memmove!(__aeabi_memmove4);
    memmove!(__aeabi_memmove8);


    macro_rules! memset {
        ($name:ident) => {
            #[no_mangle]
            pub unsafe extern fn $name(dst: *mut u8, n: usize, val: c_int) {
                super::memset(dst, val, n);
            }
        }
    }
    memset!(__aeabi_memset);
    memset!(__aeabi_memset4);
    memset!(__aeabi_memset8);

    #[no_mangle] pub fn __aeabi_unwind_cpp_pr1() { }
    #[no_mangle] pub fn __aeabi_unwind_cpp_pr0() { }
}
