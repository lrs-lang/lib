// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::str::{longest_sequence};
use {Debug, UpperHex};
use io::{Write};
use ty_one::byte_str::{ByteStr};
use ty_one::rmo::{AsRef};
use impls::str::{debug_str_no_quotes};

impl Debug for ByteStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let mut bytes = self.as_ref();
        try!(w.write_all(b"\""));
        while bytes.len() > 0 {
            let remove = {
                let as_str = longest_sequence(bytes);
                try!(debug_str_no_quotes(as_str, w));
                as_str.len()
            };
            bytes = &bytes[remove..];
            if bytes.len() > 0 {
                try!(w.write_all(b"\\x"));
                try!(UpperHex::fmt(&bytes[0], w));
                bytes = &bytes[1..];
            }
        }
        try!(w.write_all(b"\""));
        Ok(())
    }
}
