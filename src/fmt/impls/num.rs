// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use io::{Write};
use core::{num, cmp};
use {Debug, Display, UpperHex, LowerHex};

impl Debug for i8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for i16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for i32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }
impl Debug for isize { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as i64), w) } }

impl Debug for u8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for u16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for u32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }
impl Debug for usize { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(&(*self as u64), w) } }

const MAX_WIDTH_64: usize = 20; // -9223372036854775808 // 18446744073709551615

impl Debug for i64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        let val = *self;
        if val == num::i64::MIN {
            return w.write_all(b"-9223372036854775808").ignore_ok();
        }
        let mut buf = [0; MAX_WIDTH_64];
        let n = format_u64(&mut buf, val.abs() as u64);
        let buf = if val < 0 {
            buf[n-1] = b'-';
            &buf[n-1..]
        } else {
            &buf[n..]
        };
        w.write_all(buf).ignore_ok()
    }
}

impl Debug for u64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        let mut buf = [0; MAX_WIDTH_64];
        let n = format_u64(&mut buf, *self);
        w.write_all(&buf[n..]).ignore_ok()
    }
}

fn format_u64(buf: &mut [u8; MAX_WIDTH_64], mut val: u64) -> usize {
    let mut i = MAX_WIDTH_64;

    loop {
        i -= 1;
        buf[i] = b'0' + (val % 10) as u8;
        val /= 10;
        if val == 0 {
            break;
        }
    }

    i
}

impl Display for i8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for i64   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for isize { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }

impl Display for u8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for u64   { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }
impl Display for usize { fn fmt<W: Write>(&self, w: &mut W) -> Result { Debug::fmt(self, w) } }

fn hex_fmt<W: Write>(mut val: u64, w: &mut W, base: u8) -> Result {
    let mut buf = [0; 16];
    let width = cmp::max(64 - val.leading_zeros() + 3, 4) / 4;
    for i in 0..width {
        let rem = (val % 16) as u8;
        val /= 16;
        buf[width - i - 1] = match rem {
            0...9 => b'0' + rem,
            _ => base + rem - 10,
        };
    }
    try!(w.write_all(&buf[..width]));
    Ok(())
}

impl LowerHex for u64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        hex_fmt(*self, w, b'a')
    }
}

impl UpperHex for u64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        hex_fmt(*self, w, b'A')
    }
}

impl LowerHex for u8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for u16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for u32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }
impl LowerHex for usize { fn fmt<W: Write>(&self, w: &mut W) -> Result { LowerHex::fmt(&(*self as u64), w) } }

impl UpperHex for u8    { fn fmt<W: Write>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for u16   { fn fmt<W: Write>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for u32   { fn fmt<W: Write>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
impl UpperHex for usize { fn fmt<W: Write>(&self, w: &mut W) -> Result { UpperHex::fmt(&(*self as u64), w) } }
