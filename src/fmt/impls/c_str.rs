// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use ty_one::c_str::{CStr};
use {Debug};
use io::{Write};
use ty_one::byte_str::{AsByteStr};

impl Debug for CStr {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        Debug::fmt(self.as_byte_str(), w)
    }
}

