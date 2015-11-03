// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use str_one::no_null_str::{NoNullStr};
use str_two::no_null_string::{NoNullString};
use {ToOwned};
use alloc::{Allocator};

impl<H> ToOwned<H> for NoNullStr
    where H: Allocator,
{
    type Owned = NoNullString<H>;
    fn to_owned_with_pool(&self, pool: H::Pool) -> Result<NoNullString<H>> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_owned_with_pool(pool).map(|o| unsafe {
            NoNullString::from_bytes_unchecked(o)
        })
    }
}
