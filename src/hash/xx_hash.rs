// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


// Copyright (C) 2012-2014, Yann Collet.
// BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
// 
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


//! This is an implementation of xxHash by Yann Collet. The original implementation can be
//! found under [0] and is licensed under the two clause BSD license shown above. This
//! file is dual-licensed under MLP 2.0 and the two clause BSD license.
//!
//! [0]: https://github.com/Cyan4973/xxHash
//!
//! NOTE: This code has been very carefully optimized to avoid all bounds checks and
//! produce good assembly on x86_64. DO NOT CHANGE THE CODE IN THIS FILE FOR NOW.

use base::prelude::*;
use core::{mem};
use wrapping::{W32, W64};
use {Hasher};

pub const PRIME32_1 : u32 = 2654435761;
pub const PRIME32_2 : u32 = 2246822519;
pub const PRIME32_3 : u32 = 3266489917;
pub const PRIME32_4 : u32 = 668265263;
pub const PRIME32_5 : u32 = 374761393;

pub const PRIME64_1 : u64 = 11400714785074694791;
pub const PRIME64_2 : u64 = 14029467366897019727;
pub const PRIME64_3 : u64 = 1609587929392839161;
pub const PRIME64_4 : u64 = 9650029242287828579;
pub const PRIME64_5 : u64 = 2870177450012600261;

#[cfg(any(target_arch = "x86_64", target_arch = "x86", unaligned_access))]
unsafe fn read_u32(input: &[u8]) -> W32 {
    W32(*(input.as_ptr() as *const u32))
}

#[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        not(unaligned_access)))]
unsafe fn read_u32(input: &[u8]) -> W32 {
    W32(::core::ptr::read(input.as_ptr() as *const u32))
}

/// Loads an u32 and advances the input slice by 4 bytes. UB if input is shorter.
unsafe fn consume_u32(input: &mut &[u8]) -> W32 {
    let res = read_u32(*input);
    *input = input.unchecked_slice_from(4);
    res
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86", unaligned_access))]
unsafe fn read_u64(input: &[u8]) -> W64 {
    W64(*(input.as_ptr() as *const u64))
}

#[cfg(all(
        any(target_arch = "arm", target_arch = "aarch64"),
        not(unaligned_access)))]
unsafe fn read_u64(input: &[u8]) -> W64 {
    W64(::core::ptr::read(input.as_ptr() as *const u64))
}

/// Loads an u64 and advances the input slice by 8 bytes. UB if input is shorter.
unsafe fn consume_u64(input: &mut &[u8]) -> W64 {
    let res = read_u64(*input);
    *input = input.unchecked_slice_from(8);
    res
}

pub fn u32hash_bytes(input: &[u8], seed: u32) -> u32 {
    // Hack for better code generation.
    let mut input = input;
    let seed = W32(seed);
    let mut hash = W32(input.len() as u32);

    hash = hash + if input.len() >= 16 {
        let mut v1 = seed + PRIME32_1 + PRIME32_2;
        let mut v2 = seed + PRIME32_2;
        let mut v3 = seed + 0;
        let mut v4 = seed - PRIME32_1;
        while input.len() >= 16 {
            // XXX: When changing this code, check the assembly afterwards. The current
            // version looks optimal on x86_64.
            macro_rules! l {
                ($var:ident) => {{
                    $var = $var + (unsafe { consume_u32(&mut input) } * PRIME32_2);
                    $var = $var.rotate_left(13);
                    $var = $var * PRIME32_1;
                }}
            }
            l!(v1);
            l!(v2);
            l!(v3);
            l!(v4);
        }
        v1.rotate_left(1) + v2.rotate_left(7) + v3.rotate_left(12) + v4.rotate_left(18)
    } else {
        seed + PRIME32_5
    };

    while input.len() >= 4 {
        hash = hash + (unsafe { consume_u32(&mut input) } * PRIME32_3);
        hash = hash.rotate_left(17) * PRIME32_4;
    }

    while input.len() > 0 {
        hash = hash + (W32(input[0] as u32) * PRIME32_5);
        hash = hash.rotate_left(11) * PRIME32_1;
        input = &input[1..];
    }

    hash = hash ^ (hash >> 15);
    hash = hash * PRIME32_2;
    hash = hash ^ (hash >> 13);
    hash = hash * PRIME32_3;
    hash = hash ^ (hash >> 16);

    hash.0
}

pub fn u64hash_bytes(input: &[u8], seed: u64) -> u64 {
    // Hack for better code generation.
    let mut input = input;
    let seed = W64(seed);
    let mut hash = W64(input.len() as u64);

    hash = hash + if input.len() >= 32 {
        let mut v1 = seed + PRIME64_1 + PRIME64_2;
        let mut v2 = seed + PRIME64_2;
        let mut v3 = seed + 0;
        let mut v4 = seed - PRIME64_1;
        while input.len() >= 32 {
            macro_rules! l {
                ($var:ident) => {{
                    $var = $var + (unsafe { consume_u64(&mut input) } * PRIME64_2);
                    $var = $var.rotate_left(31);
                    $var = $var * PRIME64_1;
                }}
            }
            l!(v1);
            l!(v2);
            l!(v3);
            l!(v4);
        }

        let mut hash = v1.rotate_left(1) + v2.rotate_left(7) +
                            v3.rotate_left(12) + v4.rotate_left(18);

        macro_rules! r {
            ($var:ident) => {{
                $var = $var * PRIME64_2;
                $var = $var.rotate_left(31);
                $var = $var * PRIME64_1;
                hash = hash ^ $var;
                hash = hash * PRIME64_1 + PRIME64_4;
            }}
        }
        r!(v1);
        r!(v2);
        r!(v3);
        r!(v4);

        hash
    } else {
        seed + PRIME64_5
    };

    while input.len() >= 8 {
        let mut k1 = unsafe { consume_u64(&mut input) };
        k1 = k1 * PRIME64_2;
        k1 = k1.rotate_left(31);
        k1 = k1 * PRIME64_1;
        hash = hash ^ k1;
        hash = hash.rotate_left(27) * PRIME64_1 + PRIME64_4;
    }

    if input.len() >= 4 {
        hash = hash ^ (unsafe { W64(consume_u32(&mut input).0 as u64) } * PRIME64_1);
        hash = hash.rotate_left(23) * PRIME64_2 + PRIME64_3;
    }

    while input.len() > 0 {
        hash = hash ^ (W64(input[0] as u64) * PRIME64_5);
        hash = hash.rotate_left(11) * PRIME64_1;
        input = &input[1..];
    }

    hash = hash ^ (hash >> 33);
    hash = hash * PRIME64_2;
    hash = hash ^ (hash >> 29);
    hash = hash * PRIME64_3;
    hash = hash ^ (hash >> 32);

    hash.0
}

// LLVM won't inline and optimize the special cases below unless the functions above are
// marked with inline or something like that. Since we don't want to inline the function
// above in general, the implementations below have been inlined by hand and should always
// produce the same output as the functions above, just faster.

pub fn u32hash_u8(input: u8, seed: u32) -> u32 {
    let mut hash = W32(1);

    hash = hash + seed + PRIME32_5;
    hash = hash + (W32(input as u32) * PRIME32_5);
    hash = hash.rotate_left(11) * PRIME32_1;
    hash = hash ^ (hash >> 15);
    hash = hash * PRIME32_2;
    hash = hash ^ (hash >> 13);
    hash = hash * PRIME32_3;
    hash = hash ^ (hash >> 16);

    hash.0
}

pub fn u64hash_u8(input: u8, seed: u64) -> u64 {
    let mut hash = W64(1);

    hash = hash + seed + PRIME64_5;
    hash = hash ^ (W64(input as u64) * PRIME64_5);
    hash = hash.rotate_left(11) * PRIME64_1;
    hash = hash ^ (hash >> 33);
    hash = hash * PRIME64_2;
    hash = hash ^ (hash >> 29);
    hash = hash * PRIME64_3;
    hash = hash ^ (hash >> 32);

    hash.0
}

pub fn u32hash_u16(input: u16, seed: u32) -> u32 {
    let input: [u8; 2] = unsafe { mem::cast(input) };

    let mut hash = W32(2);

    hash = hash + seed + PRIME32_5;
    hash = hash + (W32(input[0] as u32) * PRIME32_5);
    hash = hash.rotate_left(11) * PRIME32_1;
    hash = hash + (W32(input[1] as u32) * PRIME32_5);
    hash = hash.rotate_left(11) * PRIME32_1;
    hash = hash ^ (hash >> 15);
    hash = hash * PRIME32_2;
    hash = hash ^ (hash >> 13);
    hash = hash * PRIME32_3;
    hash = hash ^ (hash >> 16);

    hash.0
}

pub fn u64hash_u16(input: u16, seed: u64) -> u64 {
    let input: [u8; 2] = unsafe { mem::cast(input) };

    let mut hash = W64(2 + seed);

    hash = hash + PRIME64_5;
    hash = hash ^ (W64(input[0] as u64) * PRIME64_5);
    hash = hash.rotate_left(11) * PRIME64_1;
    hash = hash ^ (W64(input[1] as u64) * PRIME64_5);
    hash = hash.rotate_left(11) * PRIME64_1;
    hash = hash ^ (hash >> 33);
    hash = hash * PRIME64_2;
    hash = hash ^ (hash >> 29);
    hash = hash * PRIME64_3;
    hash = hash ^ (hash >> 32);

    hash.0
}

pub fn u32hash_u32(input: u32, seed: u32) -> u32 {
    let mut hash = W32(4);

    hash = hash + seed + PRIME32_5;
    hash = hash + (W32(input) * PRIME32_3);
    hash = hash.rotate_left(17) * PRIME32_4;
    hash = hash ^ (hash >> 15);
    hash = hash * PRIME32_2;
    hash = hash ^ (hash >> 13);
    hash = hash * PRIME32_3;
    hash = hash ^ (hash >> 16);

    hash.0
}

pub fn u64hash_u32(input: u32, seed: u64) -> u64 {
    let mut hash = W64(4);

    hash = hash + seed + PRIME64_5;
    hash = hash ^ (W64(input as u64) * PRIME64_1);
    hash = hash.rotate_left(23) * PRIME64_2 + PRIME64_3;
    hash = hash ^ (hash >> 33);
    hash = hash * PRIME64_2;
    hash = hash ^ (hash >> 29);
    hash = hash * PRIME64_3;
    hash = hash ^ (hash >> 32);

    hash.0
}

pub fn u32hash_u64(input: u64, seed: u32) -> u32 {
    let input: [W32; 2] = unsafe { mem::cast(input) };

    let mut hash = W32(8);

    hash = hash + seed + PRIME32_5;
    hash = hash + (input[0] * PRIME32_3);
    hash = hash.rotate_left(17) * PRIME32_4;
    hash = hash + (input[1] * PRIME32_3);
    hash = hash.rotate_left(17) * PRIME32_4;
    hash = hash ^ (hash >> 15);
    hash = hash * PRIME32_2;
    hash = hash ^ (hash >> 13);
    hash = hash * PRIME32_3;
    hash = hash ^ (hash >> 16);

    hash.0
}

pub fn u64hash_u64(input: u64, seed: u64) -> u64 {
    let mut k1 = W64(input);
    k1 = k1 * PRIME64_2;
    k1 = k1.rotate_left(31);
    k1 = k1 * PRIME64_1;

    let mut hash = W64(8);

    hash = hash + seed + PRIME64_5;
    hash = hash ^ k1;
    hash = hash.rotate_left(27) * PRIME64_1 + PRIME64_4;
    hash = hash ^ (hash >> 33);
    hash = hash * PRIME64_2;
    hash = hash ^ (hash >> 29);
    hash = hash * PRIME64_3;
    hash = hash ^ (hash >> 32);

    hash.0
}

#[cfg(target_pointer_width = "32")]
pub fn u32hash_usize(input: usize, seed: u32) -> u32 {
    u32hash_u32(input as u32, seed) 
}

#[cfg(target_pointer_width = "64")]
pub fn u32hash_usize(input: usize, seed: u32) -> u32 {
    u32hash_u64(input as u64, seed) 
}

#[cfg(target_pointer_width = "32")]
pub fn u64hash_usize(input: usize, seed: u64) -> u64 {
    u64hash_u32(input as u32, seed)
}

#[cfg(target_pointer_width = "64")]
pub fn u64hash_usize(input: usize, seed: u64) -> u64 {
    u64hash_u64(input as u64, seed)
}

/// Helper struct to avoid bounds checks below.
///
/// Invariant: len <= mem::size_of::<[T; 4]>()
#[derive(Pod)]
struct FourBuf<T: Pod> {
    buf: [T; 4],
    len: usize, // this could be u8 to save some bytes but then we have to add ugly
                // conversions below
}

impl<T: Pod> FourBuf<T> {
    /// Returns an empty FourBuf
    fn empty() -> FourBuf<T> {
        FourBuf {
            buf: [mem::zeroed(); 4],
            len: 0,
        }
    }

    /// Returns the initial part of the buffer that is filled with `T`.
    ///
    /// E.g. if `x` stands for a set byte and `_` for an unset byte
    ///
    ///     [xxxx][xxxx][xxx_][____]
    ///     \____  ____/
    ///          \/
    ///    returned slice
    ///       
    fn complete(&self) -> &[T] {
        unsafe { self.buf.unchecked_slice_to(self.len / mem::size_of::<T>()) }
    }

    /// Returns the filled part that is not a complete `T`.
    ///
    /// E.g.
    ///
    ///     [xxxx][xxxx][xxx_][____]
    ///                  \_/
    ///             returned slice
    ///       
    fn incomplete(&self) -> &[u8] {
        unsafe {
            self.buf.as_bytes().unchecked_slice(self.len & !(mem::size_of::<T>() - 1),
                                                self.len)
        }
    }

    /// Returns the part that is not filled.
    ///
    /// E.g.
    ///
    ///     [xxxx][xxxx][xxx_][____]
    ///                     \__  __/
    ///                        \/
    ///                  returned slice
    ///       
    fn unused(&self) -> &[u8] {
        unsafe { self.buf.as_bytes().unchecked_slice_from(self.len) }
    }

    /// Appends data to the buffer. Returns the number of bytes appended.
    fn append(&mut self, data: &[u8]) -> usize {
        unsafe {
            let res = mem::copy(self.buf.as_mut_bytes().unchecked_mut_slice_from(self.len),
                                data);
            self.len += res;
            res
        }
    }

    /// Sets the length of the buffer to zero. The data remains unchanged until new data
    /// is appended.
    fn clear(&mut self) {
        self.len = 0;
    }

    /// Returns the number of bytes in the buffer.
    fn len(&self) -> usize {
        self.len
    }
}

impl<T: Pod> Deref for FourBuf<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.buf
    }
}

#[derive(Pod)]
#[repr(C)]
pub struct U32Hasher {
    total_len: u64,
    seed:      W32,
    v1:        W32,
    v2:        W32,
    v3:        W32,
    v4:        W32,
    buf:       FourBuf<u32>,
}

impl U32Hasher {
    pub fn new(seed: u32) -> U32Hasher {
        let mut hasher: U32Hasher = unsafe { mem::uninit() };
        hasher.reset(seed);
        hasher
    }

    pub fn reset(&mut self, seed: u32) {
        let seed = W32(seed);
        self.total_len = 0;
        self.seed      = seed;
        self.v1        = seed + PRIME32_1 + PRIME32_2;
        self.v2        = seed + PRIME32_2;
        self.v3        = seed + 0;
        self.v4        = seed - PRIME32_1;
        self.buf       = FourBuf::empty();
    }

    pub fn write_bytes(&mut self, input: &[u8]) {
        let mut input = input;
        self.total_len += input.len() as u64;

        if input.len() < self.buf.unused().len() {
            self.buf.append(input);
            return;
        }

        // Due to not-so-great code generation by rustc (I believe), copying the elements
        // out generates more efficient code. Otherwise, in the loop below, they are not
        // kept in registers and copied back to their location in memory every time they
        // are changed. Probably the same thing that makes us create a temporary variable
        // for the input above.
        let mut v1 = self.v1;
        let mut v2 = self.v2;
        let mut v3 = self.v3;
        let mut v4 = self.v4;

        if self.buf.len() > 0 {
            let copied = self.buf.append(input);
            input = unsafe { input.unchecked_slice_from(copied) };
            self.buf.clear();

            macro_rules! l {
                ($var:ident, $pos:expr) => {{
                    $var = $var + W32(self.buf[$pos]) * PRIME32_2;
                    $var = $var.rotate_left(13);
                    $var = $var * PRIME32_1;
                }}
            }
            l!(v1, 0);
            l!(v2, 1);
            l!(v3, 2);
            l!(v4, 3);
        }

        while input.len() >= 16 {
            macro_rules! l {
                ($var:ident) => {{
                    $var = $var + (unsafe { consume_u32(&mut input) } * PRIME32_2);
                    $var = $var.rotate_left(13);
                    $var = $var * PRIME32_1;
                }}
            }
            l!(v1);
            l!(v2);
            l!(v3);
            l!(v4);
        }

        self.v1 = v1;
        self.v2 = v2;
        self.v3 = v3;
        self.v4 = v4;

        self.buf.append(input);
    }

    pub fn digest(&self) -> u32 {
        let mut hash = W32(self.total_len as u32);

        hash = hash + if self.total_len >= 16 {
            self.v1.rotate_left(1) + self.v2.rotate_left(7) +
                self.v3.rotate_left(12) + self.v4.rotate_left(18)
        } else {
            self.seed + PRIME32_5
        };

        for &el in self.buf.complete() {
            hash = hash + (W32(el) * PRIME32_3);
            hash = hash.rotate_left(17) * PRIME32_4;
        }

        for &el in self.buf.incomplete() {
            hash = hash + (W32(el as u32) * PRIME32_5);
            hash = hash.rotate_left(11) * PRIME32_1;
        }

        hash = hash ^ (hash >> 15);
        hash = hash * PRIME32_2;
        hash = hash ^ (hash >> 13);
        hash = hash * PRIME32_3;
        hash = hash ^ (hash >> 16);

        hash.0
    }
}

#[derive(Pod)]
#[repr(C)]
pub struct U64Hasher {
    total_len: u64,
    seed:      W64,
    v1:        W64,
    v2:        W64,
    v3:        W64,
    v4:        W64,
    buf:       FourBuf<u64>,
}

impl U64Hasher {
    pub fn new(seed: u64) -> U64Hasher {
        let mut hasher: U64Hasher = unsafe { mem::uninit() };
        hasher.reset(seed);
        hasher
    }

    pub fn reset(&mut self, seed: u64) {
        let seed = W64(seed);
        self.total_len = 0;
        self.seed      = seed;
        self.v1        = seed + PRIME64_1 + PRIME64_2;
        self.v2        = seed + PRIME64_2;
        self.v3        = seed + 0;
        self.v4        = seed - PRIME64_1;
        self.buf       = FourBuf::empty();
    }

    pub fn write_bytes(&mut self, input: &[u8]) {
        let mut input = input;
        self.total_len += input.len() as u64;

        if input.len() < self.buf.unused().len() {
            self.buf.append(input);
            return;
        }

        let mut v1 = self.v1;
        let mut v2 = self.v2;
        let mut v3 = self.v3;
        let mut v4 = self.v4;

        if self.buf.len() > 0 {
            let copied = self.buf.append(input);
            input = unsafe { input.unchecked_slice_from(copied) };
            self.buf.clear();

            macro_rules! l {
                ($var:ident, $pos:expr) => {{
                    $var = $var + W64(self.buf[$pos]) * PRIME64_2;
                    $var = $var.rotate_left(31);
                    $var = $var * PRIME64_1;
                }}
            }
            l!(v1, 0);
            l!(v2, 1);
            l!(v3, 2);
            l!(v4, 3);
        }

        while input.len() >= 32 {
            macro_rules! l {
                ($var:ident) => {{
                    $var = $var + (unsafe { consume_u64(&mut input) } * PRIME64_2);
                    $var = $var.rotate_left(31);
                    $var = $var * PRIME64_1;
                }}
            }
            l!(v1);
            l!(v2);
            l!(v3);
            l!(v4);
        }

        self.v1 = v1;
        self.v2 = v2;
        self.v3 = v3;
        self.v4 = v4;

        self.buf.append(input);
    }

    pub fn digest(&self) -> u64 {
        let mut hash = W64(self.total_len as u64);

        hash = hash + if self.total_len >= 32 {
            let mut hash = self.v1.rotate_left(1) + self.v2.rotate_left(7) +
                            self.v3.rotate_left(12) + self.v4.rotate_left(18);

            macro_rules! r {
                ($var:ident) => {{
                    let mut $var = self.$var;
                    $var = $var * PRIME64_2;
                    $var = $var.rotate_left(31);
                    $var = $var * PRIME64_1;
                    hash = hash ^ $var;
                    hash = hash * PRIME64_1 + PRIME64_4;
                }}
            }
            r!(v1);
            r!(v2);
            r!(v3);
            r!(v4);

            hash
        } else {
            self.seed + PRIME64_5
        };

        for &el in self.buf.complete() {
            let mut k1 = W64(el);
            k1 = k1 * PRIME64_2;
            k1 = k1.rotate_left(31);
            k1 = k1 * PRIME64_1;
            hash = hash ^ k1;
            hash = hash.rotate_left(27) * PRIME64_1 + PRIME64_4;
        }

        let mut buf = self.buf.incomplete();

        if buf.len() >= 4 {
            hash = hash ^ (unsafe { W64(consume_u32(&mut buf).0 as u64) } * PRIME64_1);
            hash = hash.rotate_left(23) * PRIME64_2 + PRIME64_3;
        }

        for &el in buf {
            hash = hash ^ (W64(el as u64) * PRIME64_5);
            hash = hash.rotate_left(11) * PRIME64_1;
        }

        hash = hash ^ (hash >> 33);
        hash = hash * PRIME64_2;
        hash = hash ^ (hash >> 29);
        hash = hash * PRIME64_3;
        hash = hash ^ (hash >> 32);

        hash.0
    }
}

macro_rules! impl_hasher {
    ($name:ident,
     $inner:ident,
     $digest:ident,
     $bytes:ident,
     $u8:ident,
     $u16:ident,
     $u32:ident,
     $u64:ident,
     $usize:ident
     ) => {
        /// An implementation of the xxHash algorithm.
        pub struct $name {
            data: $inner,
        }

        impl Hasher for $name {
            type Seed = $digest;
            type Digest = $digest;

            fn new<S: Into<$digest>>(seed: S) -> Self { $name { data: $inner::new(seed.into()) } }
            fn reset<S: Into<$digest>>(&mut self, seed: S) { self.data.reset(seed.into()); }

            fn write_bytes (&mut self, val: &[u8] ) { self.data.write_bytes(val); }
            fn write_u8    (&mut self, val: u8    ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_u16   (&mut self, val: u16   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_u32   (&mut self, val: u32   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_u64   (&mut self, val: u64   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_usize (&mut self, val: usize ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_i8    (&mut self, val: i8    ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_i16   (&mut self, val: i16   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_i32   (&mut self, val: i32   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_i64   (&mut self, val: i64   ) { self.write_bytes(mem::as_bytes(&val)); }
            fn write_isize (&mut self, val: isize ) { self.write_bytes(mem::as_bytes(&val)); }

            fn digest(&self) -> Self::Digest { self.data.digest() }

            fn hash_bytes <S: Into<$digest>>( val: &[u8], seed: S) -> Self::Digest { $bytes(val, seed.into()) }
            fn hash_u8    <S: Into<$digest>>( val: u8,    seed: S) -> Self::Digest { $u8(val,    seed.into()) }
            fn hash_u16   <S: Into<$digest>>( val: u16,   seed: S) -> Self::Digest { $u16(val,   seed.into()) }
            fn hash_u32   <S: Into<$digest>>( val: u32,   seed: S) -> Self::Digest { $u32(val,   seed.into()) }
            fn hash_u64   <S: Into<$digest>>( val: u64,   seed: S) -> Self::Digest { $u64(val,   seed.into()) }
            fn hash_usize <S: Into<$digest>>( val: usize, seed: S) -> Self::Digest { $usize(val, seed.into()) }
            fn hash_i8    <S: Into<$digest>>( val: i8,    seed: S) -> Self::Digest { $u8(val     as    u8,    seed.into()) }
            fn hash_i16   <S: Into<$digest>>( val: i16,   seed: S) -> Self::Digest { $u16(val    as    u16,   seed.into()) }
            fn hash_i32   <S: Into<$digest>>( val: i32,   seed: S) -> Self::Digest { $u32(val    as    u32,   seed.into()) }
            fn hash_i64   <S: Into<$digest>>( val: i64,   seed: S) -> Self::Digest { $u64(val    as    u64,   seed.into()) }
            fn hash_isize <S: Into<$digest>>( val: isize, seed: S) -> Self::Digest { $usize(val  as    usize, seed.into()) }
        }
    }
}

impl_hasher!(XxHash32, U32Hasher, u32, u32hash_bytes, u32hash_u8, u32hash_u16, u32hash_u32,
             u32hash_u64, u32hash_usize);
impl_hasher!(XxHash64, U64Hasher, u64, u64hash_bytes, u64hash_u8, u64hash_u16, u64hash_u32,
             u64hash_u64, u64hash_usize);
