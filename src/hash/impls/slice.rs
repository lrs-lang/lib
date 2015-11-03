// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};

impl<T: Hash> Hash for [T] {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        T::stateful_hash_slice(self, h);
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        T::hash_slice::<H,_>(self, seed)
    }
}

impl Hash for str {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.as_bytes().stateful_hash(h);
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash(self.as_bytes(), seed)
    }
}
