// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use str_one::c_str::{CStr};
use str_two::c_string::{CString};
use {ToOwned};
use alloc::{MemPool};

impl<H> ToOwned<H> for CStr
    where H: MemPool,
{
    type Owned = CString<H>;
    fn to_owned_with_pool(&self, pool: H) -> Result<CString<H>> {
        let bytes = self.bytes_with_null();
        bytes.to_owned_with_pool(pool).map(|o| unsafe { CString::from_bytes_unchecked(o) })
    }
}
