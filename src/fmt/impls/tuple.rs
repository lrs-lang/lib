// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use {Debug, Write};

impl<T1: Debug, T2: Debug> Debug for (T1, T2) {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "({:?}, {:?})", self.0, self.1)
    }
}

