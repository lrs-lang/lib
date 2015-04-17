// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mem::{self};
use ops::{FnOnce};
use self::Option::{None, Some};

#[derive(Copy, Eq)]
pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, Option::None)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self {
            Some(v) => Some(f(v)),
            _ => None,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Some(v) => v,
            _ => abort!(),
        }
    }

    pub fn unwrap_or(self, val: T) -> T {
        match self {
            Some(v) => v,
            _ => val,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            Some(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
