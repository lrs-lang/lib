// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Hash, Hasher};
use base::error::{Errno};
use base::into::{Into};

impl Hash for Errno {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        h.write_i32(self.0)
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash_i32(self.0, seed)
    }
}
