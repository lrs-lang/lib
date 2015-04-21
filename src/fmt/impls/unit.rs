// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use {Debug, Write};

impl Debug for () {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        w.write(b"()").ignore_ok()
    }
}
