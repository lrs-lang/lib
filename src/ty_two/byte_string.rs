// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use ty_one::byte_str::{ByteStr};
use io::{Write};
use fmt::{Debug};
use vec::{Vec};

pub struct ByteString {
    data: Vec<u8>,
}

impl ByteString {
    pub fn from_vec(v: Vec<u8>) -> ByteString {
        ByteString { data: v }
    }
}

impl Deref for ByteString {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl Debug for ByteString {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}
