// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use core::{num};
use super::{Parsable};
use {error};

macro_rules! parse {
    ($name:ident, $base:expr, [$($range:pat, {$min:expr})|+]) => {
        fn $name(bytes: &[u8], max: u64) -> Result<(u64, usize)> {
            if bytes.len() == 0 { return Err(error::InvalidArgument); }
            let mut val = 0u64; 
            for i in 0..bytes.len() {
                match bytes[i] {
                    $(
                        $range => match val.checked_mul($base)
                                                .map(|v| v + (bytes[i] - $min) as u64) {
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

parse!(bin, 2, [b'0'...b'1', {b'0'}]);
parse!(oct, 8, [b'0'...b'7', {b'0'}]);
parse!(dec, 10, [b'0'...b'9', {b'0'}]);
parse!(hex, 16, [b'0'...b'9', {b'0'} | b'a'...b'f', {b'a'} | b'A'...b'F', {b'A'}]);

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
    if bytes.len() == 0 { return Err(error::InvalidArgument); }
    match bytes[0] {
        b'+' => unsigned(&bytes[1..], max as u64).map(|(val, len)| (val as i64, len + 1)),
        b'-' => unsigned(&bytes[1..], (-min) as u64).map(|(val, len)| (-(val as i64), len + 1)),
        _    => unsigned(bytes, max as u64).map(|(val, len)| (val as i64, len)),
    }
}

macro_rules! unsigned {
    ($name:ident) => {
        impl Parsable for $name {
            fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)> {
                unsigned(bytes, num::$name::MAX as u64)
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
                signed(bytes, num::$name::MIN as i64, num::$name::MAX as i64)
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
