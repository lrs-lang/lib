// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use core::ops::{Eq};
use base::clone::{Clone};
use base::rmo::{AsRef, AsMut};
use str_one::byte_str::{ByteStr};
use fmt::{Debug, Write};
use vec::{Vec};

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

impl Clone for ByteString {
    fn clone(&self) -> Result<ByteString> {
        self.data.clone().map(|o| ByteString { data: o })
    }
}

impl Eq for ByteString {
    fn eq(&self, other: &ByteString) -> bool {
        self.data == other.data
    }
}

impl Eq<str> for ByteString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}
