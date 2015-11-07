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
    ///
    /// [field, 1]
    /// The value.
    Some(T),
}

// The code below is interesting but forces the user to add more type annotations.
// impl<T, U> Eq<Option<U>> for Option<T>
//     where T: Eq<U>
// {
//     fn eq(&self, other: &Option<U>) -> bool {
//         match (self, other) {
//             (&Some(ref s), &Some(ref o)) => s.eq(o),
//             (&None, &None) => true,
//             _ => false,
//         }
//     }
// }

impl<T> Option<T> {
    /// Replaces the value by `None` and returns the original.
    ///
    /// [return_value]
    /// Returns the original value.
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, Option::None)
    }

    /// If the value is present, replaces it by the result of a function application.
    ///
    /// [argument, f]
    /// The function that will be applied to the value.
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

    /// Unwraps the value or returns another one.
    ///
    /// [argument, val]
    /// The object that will be returned if the value is not present.
    ///
    /// [return_value]
    /// Returns the contained value or `val` if the value is not present.
    pub fn unwrap_or(self, val: T) -> T {
        match self {
            Some(v) => v,
            _ => val,
        }
    }

    /// Applies a function to the value (if any) and returns the result.
    ///
    /// [argument, f]
    /// The function that will be applied to the value.
    pub fn chain<U, F>(self, f: F) -> Option<U>
        where F: FnOnce(T) -> Option<U>
    {
        match self {
            Some(v) => f(v),
            _ => None,
        }
    }

    /// Returns an `Option` containing an immutable reference to the value (if any.)
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            Some(ref v) => Some(v),
            _ => None,
        }
    }

    /// Returns an `Option` containing an mutable reference to the value (if any.)
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            Some(ref mut v) => Some(v),
            _ => None,
        }
    }

    /// Returns whether this object contains a value.
    pub fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            _ => false,
        }
    }

    /// Returns whether this object does not contain a value.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
