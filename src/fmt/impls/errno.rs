// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::error::{Errno};
use {Write, Debug};

impl Debug for Errno {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        w.write_str(self.name()).ignore_ok()
    }
}
