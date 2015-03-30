// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::{self, FromStr};
use std::{fmt, ops, mem, cmp};
use std::borrow::{Borrow, BorrowMut, IntoCow, Cow, ToOwned};
use std::path::{Path, PathBuf};
use std::ffi::{AsOsStr, OsStr, CString, NulError, OsString};
use std::os::unix::ffi::{OsStrExt};
use std::convert::{AsMut, AsRef, Into};

/// An owned byte vector that can be interpreted as a string but doesn't necessarily
/// contain UTF-8.
///
/// This container doesn't enforce any invariants but will likely contain UTF-8.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxString {
    inner: Vec<u8>,
}

impl LinuxString {
    /// Creates a new empty string with capacity 0.
    pub fn new() -> LinuxString {
        LinuxString { inner: vec!() }
    }

    /// Creates a new empty string with the specified capacity.
    pub fn with_capacity(cap: usize) -> LinuxString {
        LinuxString { inner: Vec::with_capacity(cap) }
    }

    /// Creates a `LinuxString` from a regular `String` by moving ownership of the vector.
    pub fn from_string(string: String) -> LinuxString {
        LinuxString { inner: string.into_bytes() }
    }

    /// Creates a `LinuxString` from a byte vector.
    pub fn from_vec(vec: Vec<u8>) -> LinuxString {
        LinuxString { inner: vec }
    }

    /// Creates a `LinuxString` by copying the string.
    pub fn from_str(string: &str) -> LinuxString {
        LinuxString { inner: string.as_bytes().to_vec() }
    }

    /// Creates a `LinuxString` by copying the bytes.
    pub fn from_bytes(bytes: &[u8]) -> LinuxString {
        LinuxString { inner: bytes.to_vec() }
    }

    /// Returns an immutable reference to the contained vector.
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }

    /// Returns a mutable reference to the contained vector.
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl Into<Vec<u8>> for LinuxString {
    fn into(self) -> Vec<u8> {
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

impl AsRef<LinuxStr> for LinuxString {
    fn as_ref(&self) -> &LinuxStr {
        unsafe { mem::transmute(&self.inner[..]) }
    }
}

impl AsMut<LinuxStr> for LinuxString {
    fn as_mut(&mut self) -> &mut LinuxStr {
        unsafe { mem::transmute(&mut self.inner[..]) }
    }
}

impl AsRef<Path> for LinuxString {
    fn as_ref(&self) -> &Path {
       (*self).as_ref() 
    }
}

impl IntoCow<'static, LinuxStr> for LinuxString {
    fn into_cow(self) -> Cow<'static, LinuxStr> {
        Cow::Owned(self)
    }
}

/// An error that occured while parsing a `LinuxStr`.
pub enum ParseErr<E> {
    /// Parsing failed because the slice did not contain UTF-8.
    NotUtf8,
    /// Parsing failed because the string doesn't contain the expected data.
    Err(E),
}

/// A borrowed byte sequence that can be interpreted as a string but doesn't necessarily
/// contain UTF-8.
pub struct LinuxStr {
    inner: [u8],
}

impl LinuxStr {
    /// Tries to interpret the contained slice as UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self.as_slice()).ok()
    }

    /// Returns a reference to the contained slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// Tries to interpret the contained slice as UTF-8. If this fails it falls back to
    /// the `from_utf8_lossy` method.
    pub fn as_str_lossy<'a>(&'a self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.as_slice())
    }

    /// Like `str::parse`, but first tries to interpret the contained slice as UTF-8.
    pub fn parse<F: FromStr>(&self) -> Result<F, ParseErr<<F as FromStr>::Err>> {
        let st = match self.as_str() {
            Some(st) => st,
            _ => return Err(ParseErr::NotUtf8),
        };
        match st.parse() {
            Ok(r) => Ok(r),
            Err(e) => Err(ParseErr::Err(e)),
        }
    }

    /// Allocates a new `LinuxString` and copies the contents.
    pub fn to_linux_string(&self) -> LinuxString {
        LinuxString { inner: self.inner.to_vec() }
    }

    /// Turns the `LinuxStr` into `CString` unless there is an interior null byte.
    pub fn to_cstring(&self) -> Result<CString, NulError> {
        CString::new(self.inner.to_vec())
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

impl AsRef<Path> for LinuxStr {
    fn as_ref(&self) -> &Path {
       self.as_os_str().as_ref()
    }
}

impl ToOwned for LinuxStr {
    type Owned = LinuxString;

    fn to_owned(&self) -> LinuxString {
        LinuxString { inner: self.inner.to_vec() }
    }
}

/// A trait for converting to a borrowed linux string.
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

/// A trait for converting to a mutably borrowed linux string.
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
