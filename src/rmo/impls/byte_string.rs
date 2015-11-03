// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use str_one::byte_str::{ByteStr};
use str_two::byte_string::{ByteString};
use {ToOwned};
use alloc::{Allocator};

impl<H> ToOwned<H> for ByteStr
    where H: Allocator,
{
    type Owned = ByteString<H>;
    fn to_owned_with_pool(&self, pool: H::Pool) -> Result<ByteString<H>> {
        self.as_ref().to_owned_with_pool(pool).map(|o| ByteString::from_vec(o))
    }
}
