// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Sized};
use ops::{PartialOrd, Ordering};
use ops::Ordering::{Less, Equal, Greater};

pub trait Ord : PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
}

pub fn min<T: Ord>(one: T, two: T) -> T {
    match one.cmp(&two) {
        Less | Equal => one,
        _ => two,
    }
}

pub fn min_ref<'a, T: Ord+?Sized>(one: &'a T, two: &'a T) -> &'a T {
    match one.cmp(two) {
        Less | Equal => one,
        _ => two,
    }
}

pub fn min_mut<'a, T: Ord+?Sized>(one: &'a mut T, two: &'a mut T) -> &'a mut T {
    match one.cmp(two) {
        Less | Equal => one,
        _ => two,
    }
}

pub fn max<T: Ord>(one: T, two: T) -> T {
    match one.cmp(&two) {
        Greater | Equal => one,
        _ => two,
    }
}

pub fn max_ref<'a, T: Ord+?Sized>(one: &'a T, two: &'a T) -> &'a T {
    match one.cmp(two) {
        Greater | Equal => one,
        _ => two,
    }
}

pub fn max_mut<'a, T: Ord+?Sized>(one: &'a mut T, two: &'a mut T) -> &'a mut T {
    match one.cmp(two) {
        Greater | Equal => one,
        _ => two,
    }
}
