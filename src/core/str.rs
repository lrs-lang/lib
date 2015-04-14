// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use repr::{Repr};
use ops::{Eq};
use ::{mem};

#[lang = "str"]
impl str {
    pub fn as_ptr(&self) -> *const u8 {
        self.repr().ptr
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { mem::cast(self) }
    }
}

impl Eq for str {
    fn eq(&self, other: &str) -> bool { self.as_bytes() == other.as_bytes() }
}
