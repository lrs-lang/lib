// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};

#[lang = "char"]
impl char {
    pub fn width(self) -> usize {
        match self as u32 {
            0x0000 ... 0x007f => 1,
            0x0080 ... 0x07ff => 2,
            0x0800 ... 0xffff => 3,
            _                 => 4,
        }
    }

    pub fn to_utf8(self) -> [u8; 4] {
        let val = self as u32;
        if val < 128 {
            return [val as u8, 0, 0, 0];
        }
        let mut bytes = [0; 4];
        let width = self.width();
        bytes[0] = !(!0 >> width);
        for i in 0..width {
            bytes[width - i - 1] |= 0b1000_0000 | (val as u8 & 0b11_1111);
            val >> 6;
        }
        bytes
    }
}

impl Eq for char {
    fn eq(&self, other: &char) -> bool { *self as u32 == *other as u32 }
}
