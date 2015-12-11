// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use str_one::{ByteStr, ToCStr, CStr};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use alloc::{self, MemPool};

/// An owned byte sequence that can be interpreted as a string.
///
/// = Remarks
///
/// The Debug implementation prints strings in the formk `"string"` where all letters that
/// are not in the printable ASCII set are printed as escape sequences of the form
/// `\u{number}`.
///
/// The Display implementation writes the contained bytes directly to the output.
pub struct ByteString<Heap = alloc::Heap>
    where Heap: MemPool,
{
    data: Vec<u8, Heap>,
}

impl<H> ByteString<H>
    where H: MemPool+Default,
{
    /// Creates a new allocated `ByteString`.
    pub fn new() -> ByteString<H> {
        ByteString { data: Vec::new() }
    }
}

impl<H> ByteString<H>
    where H: MemPool,
{
    /// Creates a `ByteString` by wrapping a vector.
    ///
    /// [argument, v]
    /// The vector to be wrapped.
    pub fn from_vec(v: Vec<u8, H>) -> ByteString<H> {
        ByteString { data: v }
    }
}

impl<H> Into<Vec<u8, H>> for ByteString<H>
    where H: MemPool, 
{
    fn into(self) -> Vec<u8, H> {
        self.data
    }
}

unsafe impl<H> UndefState for ByteString<H>
    where H: MemPool, 
{
    fn num() -> usize { <Vec<u8, H> as UndefState>::num() }

    unsafe fn set_undef(val: *mut ByteString<H>, n: usize) {
        <Vec<u8, H> as UndefState>::set_undef(&mut (*val).data, n)
    }

    unsafe fn is_undef(val: *const ByteString<H>, n: usize) -> bool {
        <Vec<u8, H> as UndefState>::is_undef(&(*val).data, n)
    }
}

impl<H> Deref for ByteString<H>
    where H: MemPool,
{
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<H> DerefMut for ByteString<H>
    where H: MemPool,
{
    fn deref_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self.data.deref_mut()) }
    }
}

impl<H> Debug for ByteString<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H> Display for ByteString<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}

impl<H> AsRef<[u8]> for ByteString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &[u8] {
        &self.data[..]
    }
}

impl<H> TryAsRef<[u8]> for ByteString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(&self.data[..])
    }
}

impl<H> AsMut<[u8]> for ByteString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}

impl<H> TryAsMut<[u8]> for ByteString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut [u8]> {
        Ok(&mut self.data[..])
    }
}

impl<H> AsRef<ByteStr> for ByteString<H>
    where H: MemPool,
{
    fn as_ref(&self) -> &ByteStr {
        self.data.as_ref()
    }
}

impl<H> TryAsRef<ByteStr> for ByteString<H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&ByteStr> {
        Ok(self.data.as_ref())
    }
}

impl<H> AsMut<ByteStr> for ByteString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut ByteStr {
        self.data.as_mut()
    }
}

impl<H> TryAsMut<ByteStr> for ByteString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut ByteStr> {
        Ok(self.data.as_mut())
    }
}

impl<H> AsMut<Vec<u8, H>> for ByteString<H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut Vec<u8, H> {
        &mut self.data
    }
}

impl<H> TryAsMut<Vec<u8, H>> for ByteString<H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut Vec<u8, H>> {
        Ok(&mut self.data)
    }
}

impl<H1, H2> TryTo<ByteString<H2>> for ByteString<H1>
    where H1: MemPool,
          H2: MemPool+Default,
{
    fn try_to(&self) -> Result<ByteString<H2>> {
        self.data.try_to().map(|o| ByteString { data: o })
    }
}

impl<H> ToCStr for ByteString<H>
    where H: MemPool,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.data.to_cstr(buf)
    }
}

impl<H> TryFrom<ByteStr> for ByteString<H>
    where H: MemPool+Default,
{
    fn try_from(c: &ByteStr) -> Result<ByteString<H>> {
        let bytes: &[u8] = c.as_ref();
        bytes.try_to().map(|o| ByteString::from_vec(o))
    }
}
