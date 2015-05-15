// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Eq implementations

use core::ops::{Eq};
use base::rmo::{AsRef};
use byte_str::{ByteStr};
use c_str::{CStr};
use no_null_str::{NoNullStr};

#[inline(always)] fn bsdata(b:  &ByteStr)   -> &[u8] { b.as_ref() }
#[inline(always)] fn csdata(b:  &CStr)      -> &[u8] { b.as_ref() }
#[inline(always)] fn nnsdata(b: &NoNullStr) -> &[u8] { b.as_ref() }

#[inline(always)]
fn csnndata(b: &CStr) -> &[u8] {
    let data = csdata(b);
    &data[..data.len()-1]
}

// ByteStr == ByteStr
impl Eq for ByteStr {
    fn eq(&self, other: &ByteStr) -> bool {
        bsdata(self) == bsdata(other)
    }
}

// ByteStr == CStr
impl Eq<CStr> for ByteStr {
    fn eq(&self, other: &CStr) -> bool {
        bsdata(self) == csnndata(other)
    }
}

// ByteStr == NoNullStr
impl Eq<NoNullStr> for ByteStr {
    fn eq(&self, other: &NoNullStr) -> bool {
        bsdata(self) == nnsdata(other)
    }
}

// ByteStr == str
impl Eq<str> for ByteStr {
    fn eq(&self, other: &str) -> bool {
        bsdata(self) == other.as_bytes()
    }
}

// str == ByteStr
impl Eq<ByteStr> for str {
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_bytes() == bsdata(other)
    }
}

// ByteStr == [u8]
impl Eq<[u8]> for ByteStr {
    fn eq(&self, other: &[u8]) -> bool {
        bsdata(self) == other
    }
}

// [u8] == ByteStr
impl Eq<ByteStr> for [u8] {
    fn eq(&self, other: &ByteStr) -> bool {
        self == bsdata(other)
    }
}

// CStr == CStr
impl Eq for CStr {
    fn eq(&self, other: &CStr) -> bool {
        csdata(self) == csdata(other)
    }
}

// CStr == ByteStr
impl Eq<ByteStr> for CStr {
    fn eq(&self, other: &ByteStr) -> bool {
        csnndata(self) == bsdata(other)
    }
}

// CStr == NoNullStr
impl Eq<NoNullStr> for CStr {
    fn eq(&self, other: &NoNullStr) -> bool {
        csnndata(self) == nnsdata(other)
    }
}

// CStr == str
impl Eq<str> for CStr {
    fn eq(&self, other: &str) -> bool {
        csnndata(self) == other.as_bytes()
    }
}

// str == CStr
impl Eq<CStr> for str {
    fn eq(&self, other: &CStr) -> bool {
        self.as_bytes() == csnndata(other)
    }
}

// CStr == [u8]
impl Eq<[u8]> for CStr {
    fn eq(&self, other: &[u8]) -> bool {
        csnndata(self) == other
    }
}

// [u8] == CStr
impl Eq<CStr> for [u8] {
    fn eq(&self, other: &CStr) -> bool {
        self == csnndata(other)
    }
}

// NoNullStr == NoNullStr
impl Eq for NoNullStr {
    fn eq(&self, other: &NoNullStr) -> bool {
        nnsdata(self) == nnsdata(other)
    }
}

// NoNullStr == ByteStr
impl Eq<ByteStr> for NoNullStr {
    fn eq(&self, other: &ByteStr) -> bool {
        nnsdata(self) == bsdata(other)
    }
}

// NoNullStr == CStr
impl Eq<CStr> for NoNullStr {
    fn eq(&self, other: &CStr) -> bool {
        nnsdata(self) == csnndata(other)
    }
}

// NoNullStr == str
impl Eq<str> for NoNullStr {
    fn eq(&self, other: &str) -> bool {
        nnsdata(self) == other.as_bytes()
    }
}

// str == NoNullStr
impl Eq<NoNullStr> for str {
    fn eq(&self, other: &NoNullStr) -> bool {
        self.as_bytes() == nnsdata(other)
    }
}

// NoNullStr == [u8]
impl Eq<[u8]> for NoNullStr {
    fn eq(&self, other: &[u8]) -> bool {
        nnsdata(self) == other
    }
}

// [u8] == NoNullStr
impl Eq<NoNullStr> for [u8] {
    fn eq(&self, other: &NoNullStr) -> bool {
        self == nnsdata(other)
    }
}
