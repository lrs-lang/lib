// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{
    Eq, FnOnce, BitOr, BitAnd, BitOrAssign, BitAndAssign, BitXor, BitXorAssign,
};
use option::{Option};
use option::Option::{Some, None};

pub trait BoolExt {
    fn map<T, F>(self, f: F) -> Option<T> where F: FnOnce() -> T;
}

impl BoolExt for bool {
    fn map<T, F>(self, f: F) -> Option<T>
        where F: FnOnce() -> T,
    {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

impl Eq for bool {
    fn eq(&self, other: &bool) -> bool {
        *self == *other
    }
}

impl BitOr for bool {
    type Output = bool;
    fn bitor(self, other: bool) -> bool { self | other }
}

impl BitOrAssign for bool {
    fn bitor_assign(&mut self, other: bool) { *self |= other }
}

impl BitAnd for bool {
    type Output = bool;
    fn bitand(self, other: bool) -> bool { self & other }
}

impl BitAndAssign for bool {
    fn bitand_assign(&mut self, other: bool) { *self &= other }
}

impl BitXor for bool {
    type Output = bool;
    fn bitxor(self, other: bool) -> bool { self ^ other }
}

impl BitXorAssign for bool {
    fn bitxor_assign(&mut self, other: bool) { *self ^= other }
}
