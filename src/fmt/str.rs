// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use io::{Write};
use {Debug, Display, Result};

impl Debug for char {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        let val = *self as u32;
        if *self == '\\' {
            try!(w.write_all(b"\\\\"));
        } else if *self == '"' {
            try!(w.write_all(b"\\\""));
        } else if 31 < val && val < 127 {
            try!(w.write_all(&[val as u8]));
        } else {
            try!(w.write_all(b"\\u{"));
            try!(Debug::fmt(&val, w));
            try!(w.write_all(b"}"));
        }
        Ok(())
    }
}

impl Display for char {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        let bytes = self.to_utf8();
        try!(w.write_all(&bytes[..self.width()]));
        Ok(())
    }
}

impl Debug for str {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        try!(w.write_all(b"\""));
        for c in self {
            try!(Debug::fmt(&c, w));
        }
        try!(w.write_all(b"\""));
        Ok(())
    }
}

impl Display for str {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        try!(w.write_all(self.as_bytes()));
        Ok(())
    }
}
