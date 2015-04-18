// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use ty_one::c_str::{CStr};
use ty_one::bytes::{AsBytes};
use io::{Write};
use fmt::{Debug};
use rmo::{AsRef, AsMut, ToOwned};
use vec::{Vec};

pub struct CString {
    data: Vec<u8>,
}

impl Deref for CString {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl Debug for CString {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl AsRef<CStr> for CString {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_bytes_unchecked(&self.data[..]) }
    }
}

impl AsMut<CStr> for CString {
    fn as_mut(&mut self) -> &mut CStr {
        unsafe { CStr::from_bytes_unchecked_mut(&mut self.data[..]) }
    }
}

impl ToOwned for CStr {
    type Owned = CString;
    fn to_owned(&self) -> Result<CString> {
        Ok(CString { data: try!(self.as_bytes().to_owned()) })
    }
}
