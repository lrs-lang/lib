// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{cmp};
use {Parsable};

macro_rules! parse {
    ($name:ident, $base:expr, [$($range:pat, {$min:expr, $skip:expr})|+]) => {
        fn $name(bytes: &[u8], max: u64) -> Result<(u64, usize)> {
            let mut val = 0u64;
            for i in 0..bytes.len() {
                match bytes[i] {
                    $(
                        $range => match val.checked_mul($base)
                                                .map(|v| v + (bytes[i] - $min + $skip) as u64) {
                            Some(next) if next <= max => val = next,
                            _ => return Ok((val, i)),
                        },
                    )+
                    _ => return Ok((val, i)),
                }
            }
            Ok((val, bytes.len()))
        }
    }
}

parse!(bin, 2, [b'0'...b'1', {b'0', 0}]);
parse!(oct, 8, [b'0'...b'7', {b'0', 0}]);
parse!(dec, 10, [b'0'...b'9', {b'0', 0}]);
parse!(hex, 16, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);

fn unsigned(bytes: &[u8], max: u64) -> Result<(u64, usize)> {
    if bytes.len() < 2 { return dec(bytes, max); }
    match (bytes[0], bytes[1]) {
        (b'0', b'b') => bin(&bytes[2..], max).map(|(val, len)| (val, len + 2)),
        (b'0', b'o') => oct(&bytes[2..], max).map(|(val, len)| (val, len + 2)),
        (b'0', b'x') => hex(&bytes[2..], max).map(|(val, len)| (val, len + 2)),
        _ => dec(bytes, max),
    }
}

fn signed(bytes: &[u8], min: i64, max: i64) -> Result<(i64, usize)> {
    if bytes.len() == 0 { return Ok((0, 0)); }
    match bytes[0] {
        b'+' => unsigned(&bytes[1..], max as u64).map(|(val, len)| (val as i64, len + 1)),
        b'-' => unsigned(&bytes[1..], 0i64.wrapping_sub(min) as u64).map(|(val, len)| (-(val as i64), len + 1)),
        _    => unsigned(bytes, max as u64).map(|(val, len)| (val as i64, len)),
    }
}

macro_rules! unsigned {
    ($name:ident) => {
        impl Parsable for $name {
            fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)> {
                unsigned(bytes, $name::max() as u64)
                        .map(|(val, len)| (val as $name, len))
            }
        }
    }
}

unsigned!(u8);
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(usize);

macro_rules! signed {
    ($name:ident) => {
        impl Parsable for $name {
            fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)> {
                signed(bytes, $name::min() as i64, $name::max() as i64)
                    .map(|(val, len)| (val as $name, len))
            }
        }
    }
}

signed!(i8);
signed!(i16);
signed!(i32);
signed!(i64);
signed!(isize);

macro_rules! impl_fw {
    ($name:ident, $ty:ident, $width:expr, [$($range:pat, {$min:expr, $skip:expr})|+]) => {
        #[derive(Pod, Eq)]
        pub struct $name(pub $ty);

        impl Parsable for $name {
            fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)> {
                let bits = $ty::bits();
                let len = cmp::min(bytes.len(), bits / $width);
                let bytes = &bytes[..len];
                let mut val = 0;
                for i in 0..bytes.len() {
                    val = match bytes[i] {
                        $(
                            $range => (val * (1 << $width)) + (bytes[i] - $min + $skip) as $ty,
                        )+
                        _ => return Ok(($name(val), i)),
                    };
                }
                Ok(($name(val), bytes.len()))
            }
        }
    }
}

impl_fw!(HexU8,    u8,    4, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);
impl_fw!(HexU16,   u16,   4, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);
impl_fw!(HexU32,   u32,   4, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);
impl_fw!(HexU64,   u64,   4, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);
impl_fw!(HexUsize, usize, 4, [b'0'...b'9', {b'0', 0} | b'a'...b'f', {b'a', 10} | b'A'...b'F', {b'A', 10}]);
impl_fw!(OctU8,    u8,    3, [b'0'...b'7', {b'0', 0}]);
impl_fw!(OctU16,   u16,   3, [b'0'...b'7', {b'0', 0}]);
impl_fw!(OctU32,   u32,   3, [b'0'...b'7', {b'0', 0}]);
impl_fw!(OctU64,   u64,   3, [b'0'...b'7', {b'0', 0}]);
impl_fw!(OctUsize, usize, 3, [b'0'...b'7', {b'0', 0}]);
impl_fw!(BinU8,    u8,    1, [b'0'...b'1', {b'0', 0}]);
impl_fw!(BinU16,   u16,   1, [b'0'...b'1', {b'0', 0}]);
impl_fw!(BinU32,   u32,   1, [b'0'...b'1', {b'0', 0}]);
impl_fw!(BinU64,   u64,   1, [b'0'...b'1', {b'0', 0}]);
impl_fw!(BinUsize, usize, 1, [b'0'...b'1', {b'0', 0}]);
