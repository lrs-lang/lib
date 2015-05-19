// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Eq implementations

use core::marker::{Sized};
use core::ops::{Eq};
use base::rmo::{AsRef};
use byte_str::{ByteStr};
use c_str::{CStr};
use no_null_str::{NoNullStr};

#[inline(always)] fn bsdata(b:  &ByteStr)   -> &[u8] { b.as_ref() }
#[inline(always)] fn csdata(b:  &CStr)      -> &[u8] { b.as_ref() }
#[inline(always)] fn nnsdata(b: &NoNullStr) -> &[u8] { b.as_ref() }

impl<T: AsRef<[u8]>+?Sized> Eq<T> for ByteStr {
    fn eq(&self, other: &T) -> bool {
        bsdata(self) == other.as_ref()
    }
}

impl<T: AsRef<[u8]>+?Sized> Eq<T> for CStr {
    fn eq(&self, other: &T) -> bool {
        csdata(self) == other.as_ref()
    }
}

impl<T: AsRef<[u8]>+?Sized> Eq<T> for NoNullStr {
    fn eq(&self, other: &T) -> bool {
        nnsdata(self) == other.as_ref()
    }
}

//// ByteStr == ByteStr
//impl Eq for ByteStr {
//    fn eq(&self, other: &ByteStr) -> bool {
//        bsdata(self) == bsdata(other)
//    }
//}
//
//// ByteStr == CStr
//impl Eq<CStr> for ByteStr {
//    fn eq(&self, other: &CStr) -> bool {
//        bsdata(self) == csdata(other)
//    }
//}
//
//// ByteStr == NoNullStr
//impl Eq<NoNullStr> for ByteStr {
//    fn eq(&self, other: &NoNullStr) -> bool {
//        bsdata(self) == nnsdata(other)
//    }
//}
//
//// ByteStr == str
//impl Eq<str> for ByteStr {
//    fn eq(&self, other: &str) -> bool {
//        bsdata(self) == other.as_bytes()
//    }
//}
//

// str == ByteStr
impl Eq<ByteStr> for str {
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_bytes() == bsdata(other)
    }
}

//// ByteStr == [u8]
//impl Eq<[u8]> for ByteStr {
//    fn eq(&self, other: &[u8]) -> bool {
//        bsdata(self) == other
//    }
//}

// [u8] == ByteStr
impl Eq<ByteStr> for [u8] {
    fn eq(&self, other: &ByteStr) -> bool {
        self == bsdata(other)
    }
}

//// CStr == CStr
//impl Eq for CStr {
//    fn eq(&self, other: &CStr) -> bool {
//        csdata(self) == csdata(other)
//    }
//}
//
//// CStr == ByteStr
//impl Eq<ByteStr> for CStr {
//    fn eq(&self, other: &ByteStr) -> bool {
//        csdata(self) == bsdata(other)
//    }
//}
//
//// CStr == NoNullStr
//impl Eq<NoNullStr> for CStr {
//    fn eq(&self, other: &NoNullStr) -> bool {
//        csdata(self) == nnsdata(other)
//    }
//}
//
//// CStr == str
//impl Eq<str> for CStr {
//    fn eq(&self, other: &str) -> bool {
//        csdata(self) == other.as_bytes()
//    }
//}

// str == CStr
impl Eq<CStr> for str {
    fn eq(&self, other: &CStr) -> bool {
        self.as_bytes() == csdata(other)
    }
}

//// CStr == [u8]
//impl Eq<[u8]> for CStr {
//    fn eq(&self, other: &[u8]) -> bool {
//        csdata(self) == other
//    }
//}

// [u8] == CStr
impl Eq<CStr> for [u8] {
    fn eq(&self, other: &CStr) -> bool {
        self == csdata(other)
    }
}

//// NoNullStr == NoNullStr
//impl Eq for NoNullStr {
//    fn eq(&self, other: &NoNullStr) -> bool {
//        nnsdata(self) == nnsdata(other)
//    }
//}
//
//// NoNullStr == ByteStr
//impl Eq<ByteStr> for NoNullStr {
//    fn eq(&self, other: &ByteStr) -> bool {
//        nnsdata(self) == bsdata(other)
//    }
//}
//
//// NoNullStr == CStr
//impl Eq<CStr> for NoNullStr {
//    fn eq(&self, other: &CStr) -> bool {
//        nnsdata(self) == csdata(other)
//    }
//}
//
//// NoNullStr == str
//impl Eq<str> for NoNullStr {
//    fn eq(&self, other: &str) -> bool {
//        nnsdata(self) == other.as_bytes()
//    }
//}

// str == NoNullStr
impl Eq<NoNullStr> for str {
    fn eq(&self, other: &NoNullStr) -> bool {
        self.as_bytes() == nnsdata(other)
    }
}

//// NoNullStr == [u8]
//impl Eq<[u8]> for NoNullStr {
//    fn eq(&self, other: &[u8]) -> bool {
//        nnsdata(self) == other
//    }
//}

// [u8] == NoNullStr
impl Eq<NoNullStr> for [u8] {
    fn eq(&self, other: &NoNullStr) -> bool {
        self == nnsdata(other)
    }
}
