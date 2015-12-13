// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use alloc::{self, MemPool};

/// An owned UTF-8 string.
pub struct String<Heap: MemPool = alloc::Heap>(Vec<u8, Heap>);

impl<H> String<H>
    where H: MemPool, 
{
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8, H>) -> String<H> {
        String(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8, H>) -> Result<String<H>, Vec<u8, H>> {
        if str::from_bytes(&bytes).is_some() {
            Ok(String(bytes))
        } else {
            Err(bytes)
        }
    }

    pub fn new() -> String<H>
        where H: OutOf,
    {
        String(Vec::new())
    }

    pub fn with_capacity(cap: usize) -> Result<String<H>>
        where H: OutOf,
    {
        Ok(String(try!(Vec::with_capacity(cap))))
    }

    pub fn with_pool(pool: H) -> String<H> {
        String(Vec::with_pool(pool))
    }

    /// Returns the capacity of the string.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of available but unused bytes.
    pub fn available(&self) -> usize {
        self.0.capacity() - self.0.len()
    }

    /// Reserves memory for additional bytes.
    ///
    /// [argument, n]
    /// The number of bytes for which memory should be reserved.
    pub fn reserve(&mut self, n: usize) -> Result {
        self.0.reserve(n)
    }

    /// Minimizes the amount of used memory.
    pub fn shrink_to_fit(&mut self) -> Result {
        self.0.shrink_to_fit()
    }

    pub fn push(&mut self, s: &str) -> Result {
        self.0.push_all(s.as_bytes())
    }

    pub fn push_char(&mut self, c: char) -> Result {
        let len = c.len();
        let bytes = c.to_utf8();
        self.0.push_all(&bytes[..len])
    }
}

unsafe impl<H> UndefState for String<H>
    where H: MemPool, 
{
    fn num() -> usize { <Vec<u8, H> as UndefState>::num() }

    unsafe fn set_undef(val: *mut String<H>, n: usize) {
        <Vec<u8, H> as UndefState>::set_undef(&mut (*val).0, n)
    }

    unsafe fn is_undef(val: *const String<H>, n: usize) -> bool {
        <Vec<u8, H> as UndefState>::is_undef(&(*val).0, n)
    }
}

impl<H> Deref for String<H>
    where H: MemPool,
{
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { mem::cast(self.0.deref()) }
    }
}

impl<H> Debug for String<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H> Display for String<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}

impl<H> Into<Vec<u8, H>> for String<H>
    where H: MemPool,
{
    fn into(self) -> Vec<u8, H> {
        self.0
    }
}
