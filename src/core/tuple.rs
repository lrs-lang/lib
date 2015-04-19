// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use option::Option::{Some};
use ops::{Eq, PartialOrd, Ordering};
use cmp::{Ord};

// XXX Come up with a macro to do this or just write a compiler plugin

impl Eq for () {
    fn eq(&self, _: &()) -> bool {
        true
    }
}
impl PartialOrd for () {
    fn partial_cmp(&self, _: &()) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
impl Ord for () {
    fn cmp(&self, _: &()) -> Ordering {
        Ordering::Equal
    }
}

impl<T1: Eq>  Eq for (T1,) {
    fn eq(&self, other: &(T1,)) -> bool {
        self.0 == other.0
    }
}
impl<T1: PartialOrd> PartialOrd for (T1,) {
    fn partial_cmp(&self, other: &(T1,)) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T1: Ord> Ord for (T1,) {
    fn cmp(&self, other: &(T1,)) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T1: Eq, T2: Eq>  Eq for (T1,T2) {
    fn eq(&self, other: &(T1,T2)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
    fn ne(&self, other: &(T1,T2)) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}
impl<T1: PartialOrd, T2: PartialOrd> PartialOrd for (T1,T2) {
    fn partial_cmp(&self, other: &(T1,T2)) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => { },
            x => return x,
        }
        self.1.partial_cmp(&other.1)
    }
}
impl<T1: Ord, T2: Ord> Ord for (T1,T2) {
    fn cmp(&self, other: &(T1,T2)) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => { },
            x => return x,
        }
        self.1.cmp(&other.1)
    }
}

impl<T1: Eq, T2: Eq, T3: Eq> Eq for (T1,T2,T3) {
    fn eq(&self, other: &(T1,T2,T3)) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
    fn ne(&self, other: &(T1,T2,T3)) -> bool {
        self.0 != other.0 || self.1 != other.1 || self.2 != other.2
    }
}
impl<T1: PartialOrd, T2: PartialOrd, T3: PartialOrd> PartialOrd for (T1,T2,T3) {
    fn partial_cmp(&self, other: &(T1,T2,T3)) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => { },
            x => return x,
        }
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => { },
            x => return x,
        }
        self.2.partial_cmp(&other.2)
    }
}
impl<T1: Ord, T2: Ord, T3: Ord> Ord for (T1,T2,T3) {
    fn cmp(&self, other: &(T1,T2,T3)) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => { },
            x => return x,
        }
        match self.1.cmp(&other.1) {
            Ordering::Equal => { },
            x => return x,
        }
        self.2.cmp(&other.2)
    }
}
