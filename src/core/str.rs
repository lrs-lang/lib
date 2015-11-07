// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use option::Option::{Some, None};
use repr::{Repr};
use iter::{Iterator};
use ops::{Eq, Index, Range, RangeTo, RangeFrom, RangeFull};
use mem::{self};
use slice::{self};

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

    /// Tries to turn a byte slice into a string slice.
    ///
    /// [argument, b]
    /// The bytes that supposedly contain valid UTF-8.
    ///
    /// [return_varue]
    /// Returns the slice transmuted to a string slice if it contains valid UTF-8.
    pub fn from_bytes(b: &[u8]) -> Option<&str> {
        let longest = str::longest_sequence(b);
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

    /// Checks whether the string starts with another string.
    ///
    /// [argument, other]
    /// The string that this string should start with.
    pub fn starts_with(&self, s: &str) -> bool {
        self.as_bytes().starts_with(s.as_bytes())
    }

    /// Returns whether an index points to an UTF-8 character boundary.
    ///
    /// [argument, idx]
    /// The index.
    ///
    /// [return_value]
    /// Returns `true` if `idx` is at a character boundary. This includes
    /// `idx == self.len()`.
    ///
    /// = Remarks
    ///
    /// If `idx > self.len()`, the process is aborted.
    pub fn is_char_boundary(&self, idx: usize) -> bool {
        if idx == self.len() {
            true
        } else {
            let b = self.as_bytes()[idx];
            b & 0x80 == 0 || b & 0xC0 == 0xC0
        }
    }

    /// Returns the longest initial sequence of valid UTF-8 in a byte slice.
    ///
    /// [argument, b]
    /// The byte slice.
    pub fn longest_sequence(b: &[u8]) -> &str {
        let mut idx = 0;
        while idx < b.len() {
            let len = UTF8_CHAR_LEN[b[idx] as usize] as usize;
            if len == 1 { idx += 1; continue; }
            if len == 0 || idx + len > b.len() { break; }
            if len == 2 && b[idx+1].leading_ones() != 1 { break; }
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
}

impl Index<Range<usize>> for str {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &str {
        assert!(index.start <= index.end);
        assert!(self.is_char_boundary(index.start));
        assert!(self.is_char_boundary(index.end));
        let len = index.end - index.start;
        unsafe {
            let start = self.as_ptr().add(index.start);
            mem::cast(slice::from_ptr(start, len))
        }
    }
}

impl Index<RangeTo<usize>> for str {
    type Output = str;

    fn index(&self, index: RangeTo<usize>) -> &str {
        self.index(0..index.end)
    }
}

impl Index<RangeFrom<usize>> for str {
    type Output = str;

    fn index(&self, index: RangeFrom<usize>) -> &str {
        self.index(index.start..self.len())
    }
}

impl Index<RangeFull> for str {
    type Output = str;

    fn index(&self, _: RangeFull) -> &str {
        self
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

/// Iterator over characters and their UTF-8 lengths in a string.
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
