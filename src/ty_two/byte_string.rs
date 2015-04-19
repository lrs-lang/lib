// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use ty_one::byte_str::{ByteStr};
use ty_one::clone::{Clone};
use io::{Write};
use fmt::{Debug};
use vec::{Vec};
use rmo::{AsRef, AsMut, ToOwned};

pub struct ByteString {
    data: Vec<u8>,
}

impl ByteString {
    pub fn from_vec(v: Vec<u8>) -> ByteString {
        ByteString { data: v }
    }
}

impl Deref for ByteString {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self.data.deref_mut()) }
    }
}

impl Debug for ByteString {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl AsRef<ByteStr> for ByteString {
    fn as_ref(&self) -> &ByteStr {
        self.deref()
    }
}

impl AsMut<ByteStr> for ByteString {
    fn as_mut(&mut self) -> &mut ByteStr {
        self.deref_mut()
    }
}

impl ToOwned for ByteStr {
    type Owned = ByteString;
    fn to_owned(&self) -> Result<ByteString> {
        self.as_ref().to_owned().map(|o| ByteString { data: o })
    }
}

impl Clone for ByteString {
    fn clone(&self) -> Result<ByteString> {
        self.data.clone().map(|o| ByteString { data: o })
    }
}
