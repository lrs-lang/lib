// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use io::{Write};
use {Debug, Display};

fn debug_char_no_quotes<W: Write>(c: char, w: &mut W, esc_double: bool,
                                         esc_single: bool) -> Result {
    let val = c as u32;
    if c == '\\' {
        try!(w.write_all(b"\\\\"));
    } else if esc_double && c == '"' {
        try!(w.write_all(b"\\\""));
    } else if esc_single && c == '\'' {
        try!(w.write_all(b"\\'"));
    } else if 31 < val && val < 127 {
        try!(w.write_all(&[val as u8]));
    } else {
        try!(w.write_all(b"\\u{"));
        try!(Debug::fmt(&val, w));
        try!(w.write_all(b"}"));
    }
    Ok(())
}

impl Debug for char {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        try!(w.write_all(b"'"));
        try!(debug_char_no_quotes(*self, w, false, true));
        try!(w.write_all(b"'"));
        Ok(())
    }
}

impl Display for char {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        let bytes = self.to_utf8();
        try!(w.write_all(&bytes[..self.len()]));
        Ok(())
    }
}

pub fn debug_str_no_quotes<W: Write>(s: &str, w: &mut W) -> Result {
    for c in s {
        try!(debug_char_no_quotes(c, w, true, false));
    }
    Ok(())
}

impl Debug for str {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        try!(w.write_all(b"\""));
        try!(debug_str_no_quotes(self, w));
        try!(w.write_all(b"\""));
        Ok(())
    }
}

impl Display for str {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        try!(w.write_all(self.as_bytes()));
        Ok(())
    }
}
