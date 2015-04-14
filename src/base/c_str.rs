// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fmt, mem};
use std::ops::{Deref};
use std::borrow::{ToOwned, Borrow};

use cty::{c_char};
use string::{AsLinuxStr};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CString {
    inner: Vec<u8>,
}

impl CString {
    pub unsafe fn from_vec_unchecked(mut vec: Vec<u8>) -> CString {
        vec.push(0);
        CString { inner: vec }
    }
}

impl Deref for CString {
    type Target = CStr;

    fn deref(&self) -> &CStr {
        unsafe { mem::transmute(&self.inner[..]) }
    }
}

impl Borrow<CStr> for CString {
    fn borrow(&self) -> &CStr {
        self.deref()
    }
}

impl fmt::Debug for CString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_linux_str().fmt(fmt)
    }
}

pub struct CStr {
    inner: [u8]
}

impl CStr {
    pub unsafe fn from_nt_slice(slice: &[u8]) -> &CStr {
        mem::transmute(slice)
    }

    pub unsafe fn from_nt_slice_mut(slice: &mut [u8]) -> &mut CStr {
        mem::transmute(slice)
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr() as *const c_char
    }

    pub fn as_slice(&self) -> &[u8] {
        let bytes = self.as_slice_with_null();
        &bytes[..bytes.len() - 1]
    }

    pub fn as_slice_with_null(&self) -> &[u8] {
        unsafe { mem::transmute(&self.inner) }
    }
}

impl fmt::Debug for CStr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_linux_str().fmt(fmt)
    }
}

impl ToOwned for CStr {
    type Owned = CString;

    fn to_owned(&self) -> CString {
        CString { inner: self.as_slice_with_null().to_vec() }
    }
}
