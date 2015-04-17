// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use io::{Write};
use fmt::{Debug, Display};
use vec::{Vec};

pub struct String {
    data: Vec<u8>,
}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl Debug for String {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl Display for String {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}
