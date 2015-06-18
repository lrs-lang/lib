// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Hash, Hasher};
use base::into::{Into};

impl Hash for () {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        h.write_u8(0)
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash_u8(0, seed)
    }
}

impl<T0: Hash> Hash for (T0,) {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h)
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash(&self.0, seed)
    }
}

impl<T0, T1> Hash for (T0,T1)
    where T0: Hash,
          T1: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
    }
}

impl<T0, T1, T2> Hash for (T0,T1,T2)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
    }
}

impl<T0, T1, T2, T3> Hash for (T0,T1,T2,T3)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
          T3: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
        self.3.stateful_hash(h);
    }
}

impl<T0, T1, T2, T3, T4> Hash for (T0,T1,T2,T3,T4)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
          T3: Hash,
          T4: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
        self.3.stateful_hash(h);
        self.4.stateful_hash(h);
    }
}

impl<T0, T1, T2, T3, T4, T5> Hash for (T0,T1,T2,T3,T4,T5)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
          T3: Hash,
          T4: Hash,
          T5: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
        self.3.stateful_hash(h);
        self.4.stateful_hash(h);
        self.5.stateful_hash(h);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Hash for (T0,T1,T2,T3,T4,T5,T6)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
          T3: Hash,
          T4: Hash,
          T5: Hash,
          T6: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
        self.3.stateful_hash(h);
        self.4.stateful_hash(h);
        self.5.stateful_hash(h);
        self.6.stateful_hash(h);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Hash for (T0,T1,T2,T3,T4,T5,T6,T7)
    where T0: Hash,
          T1: Hash,
          T2: Hash,
          T3: Hash,
          T4: Hash,
          T5: Hash,
          T6: Hash,
          T7: Hash,
{
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        self.0.stateful_hash(h);
        self.1.stateful_hash(h);
        self.2.stateful_hash(h);
        self.3.stateful_hash(h);
        self.4.stateful_hash(h);
        self.5.stateful_hash(h);
        self.6.stateful_hash(h);
        self.7.stateful_hash(h);
    }
}
