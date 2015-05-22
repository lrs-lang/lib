// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef};
use base::default::{Default};
use str_one::byte_str::{ByteStr};
use str_two::byte_string::{ByteString};
use {ToOwned};
use alloc::{Allocator};

impl<H> ToOwned<H> for ByteStr
    where H: Allocator,
          H::Pool: Default,
{
    type Owned = ByteString<H>;
    fn to_owned(&self) -> Result<ByteString<H>> {
        self.as_ref().to_owned().map(|o| ByteString::from_vec(o))
    }
}
