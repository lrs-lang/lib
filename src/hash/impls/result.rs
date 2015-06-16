// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};

impl<T: Hash, E: Hash> Hash for Result<T, E> {
    fn hash<H: Hasher>(&self, seed: H::Seed) -> H::Digest {
        match *self {
            Ok(ref o) => o.hash::<H>(seed),
            Err(ref e) => e.hash::<H>(seed),
        }
    }

    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            Ok(ref o) => o.stateful_hash(h),
            Err(ref e) => e.stateful_hash(h),
        }
    }
}
