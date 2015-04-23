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

pub struct ByteString<'a> {
    data: Vec<'a, u8>,
}

impl<'a> ByteString<'a> {
    pub fn from_vec(v: Vec<'a, u8>) -> ByteString<'a> {
        ByteString { data: v }
    }
}

impl<'a> Deref for ByteString<'a> {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<'a> DerefMut for ByteString<'a> {
    fn deref_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self.data.deref_mut()) }
    }
}

impl<'a> Debug for ByteString<'a> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<'a> AsRef<ByteStr> for ByteString<'a> {
    fn as_ref(&self) -> &ByteStr {
        self.deref()
    }
}

impl<'a> AsMut<ByteStr> for ByteString<'a> {
    fn as_mut(&mut self) -> &mut ByteStr {
        self.deref_mut()
    }
}

impl Clone for ByteString<'static> {
    fn clone(&self) -> Result<ByteString<'static>> {
        self.data.clone().map(|o| ByteString { data: o })
    }
}

impl<'a> Eq for ByteString<'a> {
    fn eq(&self, other: &ByteString<'a>) -> bool {
        self.data == other.data
    }
}

impl<'a> Eq<str> for ByteString<'a> {
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}
