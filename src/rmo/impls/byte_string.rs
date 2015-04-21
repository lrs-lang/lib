// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef};
use str_one::byte_str::{ByteStr};
use str_two::byte_string::{ByteString};
use {ToOwned};

impl ToOwned for ByteStr {
    type Owned = ByteString;
    fn to_owned(&self) -> Result<ByteString> {
        self.as_ref().to_owned().map(|o| ByteString::from_vec(o))
    }
}
