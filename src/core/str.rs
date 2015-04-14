// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use option::Option::{Some, None};
use repr::{Repr};
use iter::{Iterator};
use ops::{Eq};
use mem::{self};

#[lang = "str"]
impl str {
    pub fn as_ptr(&self) -> *const u8 {
        self.repr().ptr
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }

    //pub fn chars<'a>(&'a self) -> Chars<'a> {
    //    Chars { data: self.as_bytes() }
    //}

    pub fn chars_width<'a>(&'a self) -> CharsWidth<'a> {
        CharsWidth { data: self.as_bytes() }
    }
}

impl Eq for str {
    fn eq(&self, other: &str) -> bool { self.as_bytes() == other.as_bytes() }
}


pub static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl<'a> Iterator for &'a str {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let mut cw = CharsWidth { data: self.as_bytes() };
        let res = cw.next().map(|(c, _)| c);
        *self = unsafe { mem::cast(cw.data) };
        res
    }
}

pub struct CharsWidth<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for CharsWidth<'a> {
    type Item = (char, usize);
    fn next(&mut self) -> Option<(char, usize)> {
        if self.data.len() == 0 {
            return None;
        }
        let width = UTF8_CHAR_WIDTH[self.data[0] as usize] as usize;
        let c = unsafe { bytes_to_char(&self.data[..width]) };
        self.data = &self.data[width..];
        Some((c, width))
    }
}

unsafe fn bytes_to_char(b: &[u8]) -> char {
    if b.len() == 1 { return b[0] as char; }
    let mut val = (b[0] & (!0 >> b.len())) as u32;
    for &byte in &b[1..] {
        val = (val << 6) | (byte & 0b0011_1111) as u32;
    }
    mem::cast(val)
}
