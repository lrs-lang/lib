// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use ty_one::path::{Path};
use ty_one::byte_str::{AsByteStr};
use {Debug, Write};

impl Debug for Path {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.as_byte_str().fmt(w)
    }
}
