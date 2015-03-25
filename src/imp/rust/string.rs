// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::{self, FromStr};
use std::{self, fmt, ops, mem, cmp};
use std::borrow::{Borrow, BorrowMut, IntoCow, Cow, ToOwned};
use std::path::{AsPath, Path, PathBuf};
use std::ffi::{IntoBytes, AsOsStr, OsStr, CString, NulError, OsString};
use std::os::unix::ffi::{OsStrExt};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxString {
    inner: Vec<u8>,
}

impl LinuxString {
    pub fn new() -> LinuxString {
        LinuxString { inner: vec!() }
    }

    pub fn with_capacity(cap: usize) -> LinuxString {
        LinuxString { inner: Vec::with_capacity(cap) }
    }

    pub fn from_string(string: String) -> LinuxString {
        LinuxString { inner: string.into_bytes() }
    }

    pub fn from_vec(vec: Vec<u8>) -> LinuxString {
        LinuxString { inner: vec }
    }

    pub fn from_str(string: &str) -> LinuxString {
        LinuxString { inner: string.as_bytes().to_vec() }
    }

    pub fn from_bytes(bytes: &[u8]) -> LinuxString {
        LinuxString { inner: bytes.to_vec() }
    }

    pub fn to_cstring(&self) -> Result<CString, NulError> {
        CString::new(self.inner.clone())
    }

    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl IntoBytes for LinuxString {
    fn into_bytes(self) -> Vec<u8> {
        self.inner
    }
}

impl AsOsStr for LinuxString {
    fn as_os_str(&self) -> &OsStr {
        Borrow::<LinuxStr>::borrow(self).as_os_str()
    }
}

impl ops::Deref for LinuxString {
    type Target = LinuxStr;

    fn deref(&self) -> &LinuxStr {
        unsafe { mem::transmute(&self.inner[..]) }
    }
}

impl ops::DerefMut for LinuxString {
    fn deref_mut(&mut self) -> &mut LinuxStr {
        unsafe { mem::transmute(&mut self.inner[..]) }
    }
}

impl fmt::Debug for LinuxString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Borrow::<LinuxStr>::borrow(self).fmt(fmt)
    }
}

impl Borrow<LinuxStr> for LinuxString {
    fn borrow(&self) -> &LinuxStr {
        unsafe { mem::transmute(&self.inner[..]) }
    }
}

impl BorrowMut<LinuxStr> for LinuxString {
    fn borrow_mut(&mut self) -> &mut LinuxStr {
        unsafe { mem::transmute(&mut self.inner[..]) }
    }
}

impl IntoCow<'static, LinuxStr> for LinuxString {
    fn into_cow(self) -> Cow<'static, LinuxStr> {
        Cow::Owned(self)
    }
}

pub enum ParseErr<E> {
    NotUtf8,
    Err(E),
}

pub struct LinuxStr {
    inner: [u8],
}

impl LinuxStr {
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self.as_slice()).ok()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_str_lossy<'a>(&'a self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.as_slice())
    }

    pub fn parse<F: FromStr>(&self) -> std::result::Result<F, ParseErr<<F as FromStr>::Err>> {
        let st = match self.as_str() {
            Some(st) => st,
            _ => return Err(ParseErr::NotUtf8),
        };
        match st.parse() {
            Ok(r) => Ok(r),
            Err(e) => Err(ParseErr::Err(e)),
        }
    }

    pub fn to_linux_string(&self) -> LinuxString {
        LinuxString { inner: self.inner.to_vec() }
    }

    pub fn to_cstring(&self) -> Result<CString, NulError> {
        CString::new(self.inner.to_vec())
    }

    pub fn from_str(string: &str) -> &LinuxStr {
        unsafe { mem::transmute(string.as_bytes()) }
    }

    pub fn from_bytes(bytes: &[u8]) -> &LinuxStr {
        unsafe { mem::transmute(bytes) }
    }

    pub fn from_bytes_mut(bytes: &mut [u8]) -> &mut LinuxStr {
        unsafe { mem::transmute(bytes) }
    }
}

impl PartialEq for LinuxStr {
    fn eq(&self, other: &LinuxStr) -> bool {
        self.as_slice()[..].eq(&other.as_slice()[..])
    }

    fn ne(&self, other: &LinuxStr) -> bool {
        self.as_slice()[..].ne(&other.as_slice()[..])
    }
}

impl Eq for LinuxStr { }

impl PartialOrd for LinuxStr {
    fn partial_cmp(&self, other: &LinuxStr) -> Option<cmp::Ordering> {
        self.as_slice()[..].partial_cmp(&other.as_slice()[..])
    }
}

impl Ord for LinuxStr { 
    fn cmp(&self, other: &LinuxStr) -> cmp::Ordering {
        self.as_slice()[..].cmp(&other.as_slice()[..])
    }
}

impl AsOsStr for LinuxStr {
    fn as_os_str(&self) -> &OsStr {
        OsStr::from_bytes(&self.inner)
    }
}

//impl ops::Deref for LinuxStr {
//    type Target = [u8];
//
//    fn deref(&self) -> &[u8] {
//        &self.inner
//    }
//}
//
//impl ops::DerefMut for LinuxStr {
//    fn deref_mut(&mut self) -> &mut [u8] {
//        &mut self.inner
//    }
//}

impl fmt::Debug for LinuxStr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_str_lossy().fmt(fmt)
    }
}

impl<'a> IntoCow<'a, LinuxStr> for &'a LinuxStr {
    fn into_cow(self) -> Cow<'a, LinuxStr> {
        Cow::Borrowed(self)
    }
}

impl ToOwned for LinuxStr {
    type Owned = LinuxString;

    fn to_owned(&self) -> LinuxString {
        LinuxString { inner: self.inner.to_vec() }
    }
}

pub trait AsLinuxStr {
    fn as_linux_str(&self) -> &LinuxStr;
}

impl AsLinuxStr for LinuxStr { fn as_linux_str(&self) -> &LinuxStr { self } }
impl AsLinuxStr for LinuxString { fn as_linux_str(&self) -> &LinuxStr { self.borrow() } }
impl AsLinuxStr for [u8]     { fn as_linux_str(&self) -> &LinuxStr { unsafe { mem::transmute(self) } } }
impl AsLinuxStr for Vec<u8>  { fn as_linux_str(&self) -> &LinuxStr { self[..].as_linux_str() } }
impl AsLinuxStr for OsStr    { fn as_linux_str(&self) -> &LinuxStr { self.as_bytes().as_linux_str() } }
impl AsLinuxStr for OsString { fn as_linux_str(&self) -> &LinuxStr { self.as_bytes().as_linux_str() } }
impl AsLinuxStr for str      { fn as_linux_str(&self) -> &LinuxStr { self.as_bytes().as_linux_str() } }
impl AsLinuxStr for String   { fn as_linux_str(&self) -> &LinuxStr { self.as_bytes().as_linux_str() } }
impl AsLinuxStr for Path     { fn as_linux_str(&self) -> &LinuxStr { self.as_os_str().as_linux_str() } }
impl AsLinuxStr for PathBuf  { fn as_linux_str(&self) -> &LinuxStr { self.as_os_str().as_linux_str() } }

impl<'a, T: AsLinuxStr+?Sized> AsLinuxStr for &'a T {
    fn as_linux_str(&self) -> &LinuxStr { (*self).as_linux_str() }
}

pub trait AsLinuxStrMut {
    fn as_linux_str_mut(&mut self) -> &mut LinuxStr;
}

impl AsLinuxStrMut for LinuxStr { fn as_linux_str_mut(&mut self) -> &mut LinuxStr { self } }
impl AsLinuxStrMut for LinuxString { fn as_linux_str_mut(&mut self) -> &mut LinuxStr { self.borrow_mut() } }
impl AsLinuxStrMut for [u8]     { fn as_linux_str_mut(&mut self) -> &mut LinuxStr { unsafe { mem::transmute(self) } } }
impl AsLinuxStrMut for Vec<u8>  { fn as_linux_str_mut(&mut self) -> &mut LinuxStr { self[..].as_linux_str_mut() } }

pub trait AsLinuxStringMut {
    fn as_linux_string_mut(&mut self) -> &mut LinuxString;
}

impl AsLinuxStringMut for LinuxString { fn as_linux_string_mut(&mut self) -> &mut LinuxString { self } }
impl AsLinuxStringMut for Vec<u8>     { fn as_linux_string_mut(&mut self) -> &mut LinuxString { unsafe { mem::transmute(self) } } }
