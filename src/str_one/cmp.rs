// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::cmp::{Eq, PartialOrd, Ord, Ordering};
use byte_str::{ByteStr};
use c_str::{CStr};
use no_null_str::{NoNullStr};

#[inline(always)] fn dd<T: AsRef<[u8]>+?Sized>(b: &T) -> &[u8] { b.as_ref() }

impl<T: AsRef<[u8]>+?Sized> Eq<T> for ByteStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> Eq<T> for CStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> Eq<T> for NoNullStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<ByteStr> for str {
    fn eq(&self, other: &ByteStr) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<ByteStr> for [u8] {
    fn eq(&self, other: &ByteStr) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<CStr> for str {
    fn eq(&self, other: &CStr) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<CStr> for [u8] {
    fn eq(&self, other: &CStr) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<NoNullStr> for str {
    fn eq(&self, other: &NoNullStr) -> bool {
        dd(self) == dd(other)
    }
}

impl Eq<NoNullStr> for [u8] {
    fn eq(&self, other: &NoNullStr) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> PartialOrd<T> for ByteStr {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: AsRef<[u8]>+?Sized> Ord<T> for ByteStr {
    fn cmp(&self, other: &T) -> Ordering {
        dd(self).cmp(dd(other))
    }
}
