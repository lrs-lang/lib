// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{str};
use fmt::{self, Debug, Display, LowerHex, Write};
use parse::{Parse, Parsable};

mod index;

/// A borrowed byte sequence that can be interpreted as a string.
///
/// = Remarks
///
/// The Debug implementation prints strings in the formk `"string"` where all letters that
/// are not in the printable ASCII set are printed as escape sequences of the form
/// `\u{number}`.
///
/// The Display implementation writes the contained bytes directly to the output.
pub struct ByteStr([u8]);

impl ByteStr {
    /// Returns a byte string created by removing spaces and tabs from the start and end
    /// of the string.
    pub fn trim(&self) -> &ByteStr {
        let mut start = 0;
        let mut end = self.0.len();
        while start < self.0.len() {
            match self.0[start] {
                b' ' | b'\t' => { },
                _ => break,
            }
            start += 1;
        }
        while end > start {
            match self.0[end-1] {
                b' ' | b'\t' => { },
                _ => break,
            }
            end -= 1;
        }
        self.0[start..end].as_ref()
    }

    /// Returns whether the string starts with a byte slice.
    ///
    /// [argument, arg]
    /// The byte slice to be checked.
    pub fn starts_with<A: ?Sized>(&self, arg: &A) -> bool
        where A: AsRef<[u8]>,
    {
        self.0.starts_with(arg.as_ref())
    }
}

impl Deref for ByteStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for ByteStr {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Debug for ByteStr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let mut bytes: &[u8] = self.as_ref();
        try!(w.write_all(b"\""));
        while bytes.len() > 0 {
            let remove = {
                let as_str = str::longest_sequence(bytes);
                try!(fmt::impls::str::debug_str_no_quotes(as_str, w));
                as_str.len()
            };
            bytes = &bytes[remove..];
            if bytes.len() > 0 {
                try!(w.write_all(b"\\x"));
                try!(LowerHex::fmt(&bytes[0], w));
                bytes = &bytes[1..];
            }
        }
        try!(w.write_all(b"\""));
        Ok(())
    }
}

impl Display for ByteStr {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        w.write_all(&self.0).ignore_ok()
    }
}

impl Parse for ByteStr {
    fn parse<P: Parsable>(&self) -> Result<P> {
        self.0.parse()
    }
}
