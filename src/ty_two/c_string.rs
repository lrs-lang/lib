// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use core::{mem};
use ty_one::c_str::{CStr};
use io::{Write};
use fmt::{Debug};
use vec::{Vec};

pub struct CString {
    data: Vec<u8>,
}

impl Deref for CString {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl Debug for CString {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}
