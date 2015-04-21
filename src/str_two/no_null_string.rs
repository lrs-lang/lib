// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef, AsMut};
use str_one::no_null_str::{NoNullStr, AsNoNullStr, AsMutNoNullStr};
use vec::{Vec};
use fmt::{Debug, Write};

pub struct NoNullString {
    data: Vec<u8>,
}

impl NoNullString {
    /// Casts the byte vector directly to a `NoNullString` without checking it for validity.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> NoNullString {
        NoNullString { data: bytes }
    }
}

impl AsRef<NoNullStr> for NoNullString {
    fn as_ref(&self) -> &NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked(&self.data) }
    }
}

impl AsMut<NoNullStr> for NoNullString {
    fn as_mut(&mut self) -> &mut NoNullStr {
        unsafe { NoNullStr::from_bytes_unchecked_mut(&mut self.data) }
    }
}

impl AsNoNullStr for NoNullString {
    fn as_no_null_str(&self) -> Result<&NoNullStr> {
        unsafe { Ok(NoNullStr::from_bytes_unchecked(&self.data)) }
    }
}

impl AsMutNoNullStr for NoNullString {
    fn as_mut_no_null_str(&mut self) -> Result<&mut NoNullStr> {
        unsafe { Ok(NoNullStr::from_bytes_unchecked_mut(&mut self.data)) }
    }
}

impl Debug for NoNullString {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.as_ref().fmt(w)
    }
}
