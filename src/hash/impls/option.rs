// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};

impl<T: Hash> Hash for Option<T> {
    fn hash<H: Hasher>(&self, seed: H::Seed) -> H::Digest {
        match *self {
            Some(ref o) => o.hash::<H>(seed),
            _ => H::hash_u8(0, seed),
        }
    }

    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            Some(ref o) => o.stateful_hash(h),
            _ => h.write_u8(0),
        }
    }
}
