// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::{error};
use core::{mem};
use arch_fns::{memchr};
use {CStr, NoNullStr, ByteStr};

// &[u8] -> &ByteStr
// &[u8] -> Result<&NoNullStr>
// &[u8] -> Result<&CStr>
// &[i8] -> Result<&CStr>
// &mut [u8] -> &mut ByteStr
// &mut [u8] -> Result<&mut NoNullStr>
// &mut [u8] -> Result<&mut CStr>
// &mut [i8] -> Result<&mut CStr>
// &str -> &ByteStr
// &str -> Result<&NoNullStr>
// &str -> Result<&CStr>
// &ByteStr -> &[u8]
// &ByteStr -> Result<&str>
// &ByteStr -> Result<&NoNullStr>
// &ByteStr -> Result<&CStr>
// &mut ByteStr -> &mut [u8]
// &mut ByteStr -> Result<&mut NoNullStr>
// &mut ByteStr -> Result<&mut CStr>
// &NoNullStr -> &[u8]
// &NoNullStr -> &ByteStr
// &NoNullStr -> Result<&str>
// &mut NoNullStr -> &mut NoNullStr
// &CStr -> &[u8]
// &CStr -> &ByteStr
// &CStr -> &NoNullStr
// &CStr -> Result<&str>
// &mut CStr -> &mut NoNullStr

impl AsRef<ByteStr> for [u8] {
    fn as_ref(&self) -> &ByteStr {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_ref!(ByteStr, [u8]);

impl TryAsRef<NoNullStr> for [u8] {
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        if memchr(self, 0).is_none() {
            Ok(unsafe { mem::cast(self) })
        } else {
            Err(error::InvalidArgument)
        }
    }
}

impl TryAsRef<CStr> for [u8] {
    fn try_as_ref(&self) -> Result<&CStr> {
        if memchr(self, 0) == Some(self.len() - 1) {
            Ok(unsafe { mem::cast(&self[..self.len()-1]) })
        } else {
            Err(error::InvalidArgument)
        }
    }
}

impl TryAsRef<CStr> for [i8] {
    fn try_as_ref(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl AsMut<ByteStr> for [u8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_mut!(ByteStr, [u8]);

impl TryAsMut<NoNullStr> for [u8] {
    fn try_as_mut(&mut self) -> Result<&mut NoNullStr> {
        if memchr(self, 0).is_none() {
            Ok(unsafe { mem::cast(self) })
        } else {
            Err(error::InvalidArgument)
        }
    }
}

impl TryAsMut<CStr> for [i8] {
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.try_as_mut()
    }
}

impl TryAsMut<CStr> for [u8] {
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        let len = self.len();
        if memchr(self, 0) == Some(len - 1) {
            Ok(unsafe { mem::cast(&mut self[..len-1]) })
        } else {
            Err(error::InvalidArgument)
        }
    }
}

impl AsRef<ByteStr> for str {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}
impl_try_as_ref!(ByteStr, str);

impl TryAsRef<NoNullStr> for str {
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl TryAsRef<CStr> for str {
    fn try_as_ref(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl AsRef<[u8]> for ByteStr {
    fn as_ref(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_ref!([u8], ByteStr);

impl TryAsRef<str> for ByteStr {
    fn try_as_ref(&self) -> Result<&str> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl TryAsRef<NoNullStr> for ByteStr {
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl TryAsRef<CStr> for ByteStr {
    fn try_as_ref(&self) -> Result<&CStr> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl AsMut<[u8]> for ByteStr {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_mut!([u8], ByteStr);

impl TryAsMut<NoNullStr> for ByteStr {
    fn try_as_mut(&mut self) -> Result<&mut NoNullStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.try_as_mut()
    }
}

impl TryAsMut<CStr> for ByteStr {
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        let bytes: &mut [u8] = self.as_mut();
        bytes.try_as_mut()
    }
}

impl AsRef<[u8]> for NoNullStr {
    fn as_ref(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_ref!([u8], NoNullStr);

impl AsRef<ByteStr> for NoNullStr {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}
impl_try_as_ref!(ByteStr, NoNullStr);

impl TryAsRef<str> for NoNullStr {
    fn try_as_ref(&self) -> Result<&str> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl AsRef<[u8]> for CStr {
    fn as_ref(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_ref!([u8], CStr);

impl AsRef<ByteStr> for CStr {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}
impl_try_as_ref!(ByteStr, CStr);

impl AsRef<NoNullStr> for CStr {
    fn as_ref(&self) -> &NoNullStr {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_ref!(NoNullStr, CStr);

impl TryAsRef<str> for CStr {
    fn try_as_ref(&self) -> Result<&str> {
        let bytes: &[u8] = self.as_ref();
        bytes.try_as_ref()
    }
}

impl AsMut<NoNullStr> for CStr {
    fn as_mut(&mut self) -> &mut NoNullStr {
        unsafe { mem::cast(self) }
    }
}
impl_try_as_mut!(NoNullStr, CStr);
