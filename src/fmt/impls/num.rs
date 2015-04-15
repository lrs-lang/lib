// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use io::{Write};
use core::{num, cmp};
use {Debug, Display, UpperHex, LowerHex};

impl Debug for i8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for i16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for i32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for isize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }

impl Debug for u8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for u16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for u32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for usize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }

const MAX_WIDTH_64: usize = 20; // -9223372036854775808 // 18446744073709551615

impl Debug for i64 {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        let val = *self;
        let mut buf = [0; MAX_WIDTH_64];
        if val == num::i64::MIN {
            return w.write(b"-9223372036854775808").map(|_| ());
        }
        buf[0] = b'-';
        let n = format_u64(&mut buf[1..], val.abs() as u64).len();
        (if val < 0 {
            w.write_all(&buf[..n+1])
        } else {
            w.write_all(&buf[1..n+1])
        }).map(|_| ())
    }
}

impl Debug for u64 {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        let mut buf = [0; MAX_WIDTH_64];
        let n = format_u64(&mut buf[..], *self).len();
        w.write_all(&buf[..n]).map(|_| ())
    }
}

pub fn format_u64(buf: &mut [u8], mut val: u64) -> &mut [u8] {
    static WIDTH_LOOKUP: [(u8, u8); 65] = [
        (  0, 19), (255, 19), (255, 19), (255, 19), (  1, 18), (255, 18), (255, 18),
        (  2, 17), (255, 17), (255, 17), (  3, 16), (255, 16), (255, 16), (255, 16),
        (  4, 15), (255, 15), (255, 15), (  5, 14), (255, 14), (255, 14), (  6, 13),
        (255, 13), (255, 13), (255, 13), (  7, 12), (255, 12), (255, 12), (  8, 11),
        (255, 11), (255, 11), (  9, 10), (255, 10), (255, 10), (255, 10), ( 10,  9),
        (255,  9), (255,  9), ( 11,  8), (255,  8), (255,  8), ( 12,  7), (255,  7),
        (255,  7), (255,  7), ( 13,  6), (255,  6), (255,  6), ( 14,  5), (255,  5),
        (255,  5), ( 15,  4), (255,  4), (255,  4), (255,  4), ( 16,  3), (255,  3),
        (255,  3), ( 17,  2), (255,  2), (255,  2), ( 18,  1), (255,  1), (255,  1),
        (255,  1), (255,  1),
    ];
    static STEP_LOOKUP: [u64; 19] = [
        10000000000000000000, 1000000000000000000, 100000000000000000, 10000000000000000,
        1000000000000000, 100000000000000, 10000000000000, 1000000000000, 100000000000,
        10000000000, 1000000000, 100000000, 10000000, 1000000, 100000, 10000, 1000, 100,
        10
    ];
    let (idx, mut width) = WIDTH_LOOKUP[val.leading_zeros()];
    if idx != 255 && val >= STEP_LOOKUP[idx as usize] {
        width += 1;
    }
    let width = width as usize;
    for i in 0..width {
        buf[width - i - 1] = b'0' + (val % 10) as u8;
        val /= 10;
    }
    &mut buf[..width]
}

impl Display for i8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i64   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for isize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }

impl Display for u8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u64   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for usize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }

fn hex_fmt<W: Write+?Sized>(val: u64, w: &mut W, base: u8) -> Result {
    let mut buf = [0; 16];
    let width = cmp::max(64 - val.leading_zeros() + 3, 4) / 4;
    for i in 0..width {
        let rem = (val % 16) as u8;
        buf[width - i - 1] = match rem {
            0...9 => b'0' + rem,
            _ => base + rem - 10,
        };
    }
    try!(w.write_all(&buf[..width]));
    Ok(())
}

impl LowerHex for u64 {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        hex_fmt(*self, w, b'a')
    }
}

impl UpperHex for u64 {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        hex_fmt(*self, w, b'A')
    }
}

impl LowerHex for u8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for u16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for u32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for usize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }

impl UpperHex for u8    { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for u16   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for u32   { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for usize { fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
