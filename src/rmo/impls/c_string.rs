// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef};
use str_one::c_str::{CStr};
use str_two::c_string::{CString};
use {ToOwned};

impl ToOwned for CStr {
    type Owned = CString;
    fn to_owned(&self) -> Result<CString> {
        self.as_ref().to_owned().map(|o| unsafe { CString::from_bytes_unchecked(o) })
    }
}
