// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::clone::{MaybeClone};
use base::undef::{UndefState};
use base::default::{Default};
use str_one::{ByteStr, ToCStr, CStr};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use alloc::{self, Allocator};

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
    where Heap: Allocator,
{
    data: Vec<u8, Heap>,
}

impl<H> ByteString<H>
    where H: Allocator,
          H::Pool: Default,
{
    /// Creates a new allocated `ByteString`.
    pub fn new() -> ByteString<H> {
        ByteString { data: Vec::new() }
    }
}

impl<H> ByteString<H>
    where H: Allocator,
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
    where H: Allocator, 
{
    fn into(self) -> Vec<u8, H> {
        self.data
    }
}

unsafe impl<H> UndefState for ByteString<H>
    where H: Allocator, 
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
    where H: Allocator,
{
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<H> DerefMut for ByteString<H>
    where H: Allocator,
{
    fn deref_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self.data.deref_mut()) }
    }
}

impl<H> Debug for ByteString<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H> Display for ByteString<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}

impl<H> AsRef<[u8]> for ByteString<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &[u8] {
        &self.data[..]
    }
}

impl<H> AsMut<[u8]> for ByteString<H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}

impl<H> AsRef<ByteStr> for ByteString<H>
    where H: Allocator,
{
    fn as_ref(&self) -> &ByteStr {
        self.data.as_ref()
    }
}

impl<H> AsMut<ByteStr> for ByteString<H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut ByteStr {
        self.data.as_mut()
    }
}

impl<H> AsMut<Vec<u8, H>> for ByteString<H>
    where H: Allocator,
{
    fn as_mut(&mut self) -> &mut Vec<u8, H> {
        &mut self.data
    }
}

impl<H> MaybeClone for ByteString<H>
    where H: Allocator,
          H::Pool: Default,
{
    fn maybe_clone(&self) -> Result<ByteString<H>> {
        self.data.maybe_clone().map(|o| ByteString { data: o })
    }
}

impl<H> ToCStr for ByteString<H>
    where H: Allocator,
{
    fn to_cstr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        self.data.to_cstr(buf)
    }
}
