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
    /// Returns a pointer to the first byte in the string.
    pub fn as_ptr(&self) -> *const u8 {
        self.repr().ptr
    }

    /// Returns the wrapped bytes.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Checks if `b` contains vaild UTF-8 and, if so, returns it as a string.
    pub fn from_bytes(b: &[u8]) -> Option<&str> {
        let longest = longest_sequence(b);
        if longest.len() == b.len() {
            Some(longest)
        } else {
            None
        }
    }

    /// Returns an iterator over the contained characters and their UTF-8 lengths.
    pub fn chars_len<'a>(&'a self) -> CharsLen<'a> {
        CharsLen { data: self.as_bytes() }
    }
}

impl Eq for str {
    fn eq(&self, other: &str) -> bool { self.as_bytes().eq(other.as_bytes()) }
}

impl Eq<str> for [u8] {
    fn eq(&self, other: &str) -> bool { self.eq(other.as_bytes()) }
}

impl Eq<[u8]> for str {
    fn eq(&self, other: &[u8]) -> bool { self.as_bytes().eq(other) }
}

pub static UTF8_CHAR_LEN: [u8; 256] = [
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
        let mut cw = CharsLen { data: self.as_bytes() };
        let res = cw.next().map(|(c, _)| c);
        *self = unsafe { mem::cast(cw.data) };
        res
    }
}

/// See the `chars_len` documentation.
pub struct CharsLen<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for CharsLen<'a> {
    type Item = (char, usize);
    fn next(&mut self) -> Option<(char, usize)> {
        if self.data.len() == 0 {
            return None;
        }
        let len = UTF8_CHAR_LEN[self.data[0] as usize] as usize;
        let c = unsafe { bytes_to_char(&self.data[..len]) };
        self.data = &self.data[len..];
        Some((c, len))
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

/// Returns the longest initial sequence of valid UTF-8 in the `b`.
pub fn longest_sequence(b: &[u8]) -> &str {
    let mut idx = 0;
    while idx < b.len() {
        let len = UTF8_CHAR_LEN[b[idx] as usize] as usize;
        if len == 1 { idx += 1; continue; }
        if len == 0 || idx + len > b.len() { break; }
        if len == 3 {
            match (b[idx], b[idx+1], b[idx+2]) {
                (0xE0,        0xA0...0xBF, 0x80...0xBF) => { },
                (0xE1...0xEC, 0x80...0xBF, 0x80...0xBF) => { },
                (0xED,        0x80...0x9F, 0x80...0xBF) => { },
                _ => break,
            }
        }
        if len == 4 {
            match (b[idx], b[idx+1], b[idx+2], b[idx+3]) {
                (0xF0,        0x90...0xBF, 0x80...0xBF, 0x80...0xBF) => { },
                (0xF1...0xF3, 0x80...0xBF, 0x80...0xBF, 0x80...0xBF) => { },
                (0xF4,        0x80...0x8F, 0x80...0xBF, 0x80...0xBF) => { },
                _ => break,
            }
        }
        idx += len;
    }
    unsafe { mem::cast(&b[..idx]) }
}
