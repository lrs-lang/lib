// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mem::{self};
use ops::{FnOnce};
use self::Option::{None, Some};

/// An optional value.
#[derive(Copy, Eq)]
pub enum Option<T> {
    /// The value is not present.
    None,
    /// The value is present.
    Some(T),
}

impl<T> Option<T> {
    /// Replaces `self` by `None` and returns the original.
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, Option::None)
    }

    /// Applies `f` to the value (if any) and returns the result.
    pub fn map<U, F>(self, f: F) -> Option<U>
        where F: FnOnce(T) -> U,
    {
        match self {
            Some(v) => Some(f(v)),
            _ => None,
        }
    }

    /// Unwraps the value or aborts.
    pub fn unwrap(self) -> T {
        match self {
            Some(v) => v,
            _ => abort!(),
        }
    }

    /// Unwraps the value or returns `val`.
    pub fn unwrap_or(self, val: T) -> T {
        match self {
            Some(v) => v,
            _ => val,
        }
    }

    /// Returns an `Option` with containing an immutable reference to the value (if any.)
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            Some(ref v) => Some(v),
            _ => None,
        }
    }

    /// Like `as_ref`.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            Some(ref mut v) => Some(v),
            _ => None,
        }
    }

    /// Returns whether this `Option` contains a value.
    pub fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            _ => false,
        }
    }

    /// Like `is_some`.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
