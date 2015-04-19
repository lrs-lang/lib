// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::prelude::*;
use self::Result::{Ok, Err};
use error::{Errno};

#[derive(Eq)]
pub enum Result<T=(), E=Errno> {
    Ok(T),
    Err(E),
}

impl<T: Copy, E: Copy> Copy for Result<T, E> { }

impl<T, E> Result<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            _ => abort!(),
        }
    }

    pub fn map<U, F>(self, f: F) -> Result<U, E>
        where F: FnOnce(T) -> U,
    {
        match self {
            Ok(t) => Ok(f(t)),
            Err(e) => Err(e),
        }
    }

    pub fn chain<U, F>(self, f: F) -> Result<U, E>
        where F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Ok(t) => f(t),
            Err(e) => Err(e),
        }
    }

    pub fn is_ok(&self) -> bool {
        match *self {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn ignore_ok(self) -> Result<(), E> {
        match self {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
