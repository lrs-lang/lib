// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// AsRef / AsMut implementations except those that go -> [u8]

use core::{mem};
use base::rmo::{AsRef, AsMut};
use byte_str::{ByteStr};
use no_null_str::{NoNullStr};

impl AsRef<ByteStr> for [u8] {
    fn as_ref(&self) -> &ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<ByteStr> for [u8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsRef<ByteStr> for [i8] {
    fn as_ref(&self) -> &ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<ByteStr> for [i8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsRef<ByteStr> for str {
    fn as_ref(&self) -> &ByteStr {
        unsafe { mem::cast(self) }
    }
}

impl AsRef<ByteStr> for NoNullStr {
    fn as_ref(&self) -> &ByteStr {
        let bytes: &[u8] = self.as_ref();
        bytes.as_ref()
    }
}
