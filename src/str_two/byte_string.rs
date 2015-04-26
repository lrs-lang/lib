// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use core::ops::{Eq};
use base::clone::{Clone};
use base::rmo::{AsRef, AsMut};
use str_one::{ByteStr, ToCStr, CStr, AsByteStr};
use no_null_string::{NoNullString};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use alloc::{self, Allocator};

pub struct ByteString<'a, Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: Vec<'a, u8, Heap>,
}

impl<'a, H> ByteString<'a, H>
    where H: Allocator,
{
    pub fn from_vec(v: Vec<'a, u8, H>) -> ByteString<'a, H> {
        ByteString { data: v }
    }

    pub fn unwrap(self) -> Vec<'a, u8, H> {
        self.data
    }
}

impl<'a, H> Deref for ByteString<'a, H>
    where H: Allocator,
{
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<'a, H> DerefMut for ByteString<'a, H>
    where H: Allocator,
{
    fn deref_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self.data.deref_mut()) }
    }
}

impl<'a, H> Debug for ByteString<'a, H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<'a, H> Display for ByteString<'a, H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}

impl<'a, H> AsRef<[u8]> for ByteString<'a, H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[u8] {
        &self.data[..]
    }
}

impl<'a, H> AsMut<[u8]> for ByteString<'a, H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}

impl<H> Clone for ByteString<'static, H>
    where H: Allocator,
{
    fn clone(&self) -> Result<ByteString<'static, H>> {
        self.data.clone().map(|o| ByteString { data: o })
    }
}

impl<'a, H1, H2> Eq<ByteString<'a, H1>> for ByteString<'a, H2>
    where H1: Allocator,
          H2: Allocator,
{
    fn eq(&self, other: &ByteString<'a, H1>) -> bool {
        self.data == other.data
    }
}

impl<'a, H> Eq<str> for ByteString<'a, H>
    where H: Allocator,
{
    fn eq(&self, other: &str) -> bool {
        let bytes: &[u8] = self.as_ref();
        bytes.eq(other)
    }
}

impl<'a, H> Eq<ByteStr> for ByteString<'a, H>
    where H: Allocator,
{
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_ref() == other
    }
}

impl<'a, 'b, H1, H2> Eq<NoNullString<'b, H1>> for ByteString<'a, H2>
    where H1: Allocator,
          H2: Allocator,
{
    fn eq(&self, other: &NoNullString<'b, H1>) -> bool {
        self.deref() == other.as_byte_str()
    }
}

impl<'b, H> ToCStr for ByteString<'b, H>
    where H: Allocator,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.data.to_cstr(buf)
    }
}
