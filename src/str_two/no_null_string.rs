// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef, AsMut};
use str_one::no_null_str::{NoNullStr, AsNoNullStr, AsMutNoNullStr};
use vec::{Vec};
use fmt::{Debug, Write};

pub struct NoNullString<'a> {
    data: Vec<'a, u8>,
}

impl<'a> NoNullString<'a> {
    /// Casts the byte vector directly to a `NoNullString` without checking it for validity.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<'a, u8>) -> NoNullString<'a> {
        NoNullString { data: bytes }
    }
}

impl<'a> AsRef<NoNullStr> for NoNullString<'a> {
    fn as_ref(&self) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data) }
    }
}

impl<'a> AsMut<NoNullStr> for NoNullString<'a> {
    fn as_mut(&mut self) -> &mut NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked_mut(&mut self.data) }
    }
}

impl<'a> AsNoNullStr for NoNullString<'a> {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        unsafe { Ok(NoNullStr::from_bytes_unchecked(&self.data)) }
    }
}

impl<'a> AsMutNoNullStr for NoNullString<'a> {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        unsafe { Ok(NoNullStr::from_bytes_unchecked_mut(&mut self.data)) }
    }
}

impl<'a> Debug for NoNullString<'a> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.as_ref().fmt(w)
    }
}
