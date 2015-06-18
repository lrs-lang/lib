// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Hash, Hasher};
use base::into::{Into};

impl Hash for bool {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        h.write_u8(*self as u8);
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash_u8(*self as u8, seed)
    }
}
