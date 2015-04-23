// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use fmt::{Debug, Display, Write};
use vec::{Vec};

pub struct String<'a> {
    data: Vec<'a, u8>,
}

impl<'a> Deref for String<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<'a> Debug for String<'a> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<'a> Display for String<'a> {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}
