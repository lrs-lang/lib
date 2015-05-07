// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};
use ops::{PartialOrd, Ordering};
use cmp::{Ord};
use {mem};
use option::{Option};
use option::Option::{Some, None};

#[lang = "char"]
impl char {
    /// Calculates the length of the UTF-8 representation of the character.
    pub fn len(self) -> usize {
        match self as u32 {
            0x0000 ... 0x007f => 1,
            0x0080 ... 0x07ff => 2,
            0x0800 ... 0xffff => 3,
            _                 => 4,
        }
    }

    /// Converts the character to its UTF-8 representation.
    ///
    /// Only the first `len` bytes are meaningful.
    pub fn to_utf8(self) -> [u8; 4] {
        let val = self as u32;
        if val < 128 {
            return [val as u8, 0, 0, 0];
        }
        let mut bytes = [0; 4];
        let len = self.len();
        bytes[0] = !(!0 >> len);
        for i in 0..len {
            bytes[len - i - 1] |= 0b1000_0000 | (val as u8 & 0b11_1111);
            val >> 6;
        }
        bytes
    }

    pub fn from_u32(val: u32) -> Option<char> {
        if val > 0x10ffff || (val >= 0xD800 && val <= 0xDFFF) {
            None
        } else {
            Some(unsafe { mem::cast(val) })
        }
    }
}

impl Eq for char {
    fn eq(&self, other: &char) -> bool { *self as u32 == *other as u32 }
}

impl PartialOrd for char {
    fn partial_cmp(&self, other: &char) -> Option<Ordering> {
        (*self as u32).partial_cmp(&(*other as u32))
    }
}

impl Ord for char {
    fn cmp(&self, other: &char) -> Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}
