// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::prelude::*;
use self::Result::{Ok, Err};
use error::{Errno};

/// The result of an operation.
#[derive(Eq)]
pub enum Result<T=(), E=Errno> {
    /// The operation succeeded.
    Ok(T),
    /// The operation failed.
    Err(E),
}

impl<T: Copy, E: Copy> Copy for Result<T, E> { }

impl<T, E> Result<T, E> {
    /// Returns the success value or aborts if the operation did not succeed.
    pub fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            _ => abort!(),
        }
    }

    /// Replaces the success value (if any) by the result of the function.
    pub fn map<U, F>(self, f: F) -> Result<U, E>
        where F: FnOnce(T) -> U,
    {
        match self {
            Ok(t) => Ok(f(t)),
            Err(e) => Err(e),
        }
    }

    /// Applies `f` to the success value (if any) and returns its return value.
    pub fn chain<U, F>(self, f: F) -> Result<U, E>
        where F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Ok(t) => f(t),
            Err(e) => Err(e),
        }
    }

    /// Returns whether the operation succeeded.
    pub fn is_ok(&self) -> bool {
        match *self {
            Ok(_) => true,
            _ => false,
        }
    }

    /// Returns whether the operation failed.
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Replaces the success value (if any) by `()`.
    pub fn ignore_ok(self) -> Result<(), E> {
        match self {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
