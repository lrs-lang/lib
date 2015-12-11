// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{slice, mem};
use result::{Result};
use result::Result::{Ok};

/// Objects that can be immutably borrowed.
pub trait AsRef<Target: ?Sized>: TryAsRef<Target> {
    /// Borrows the object.
    fn as_ref(&self) -> &Target;
}

/// Objects that can be mutably borrowed.
pub trait AsMut<Target: ?Sized>: TryAsMut<Target> {
    /// Borrows the object.
    fn as_mut(&mut self) -> &mut Target;
}

pub trait TryAsRef<Target: ?Sized> {
    fn try_as_ref(&self) -> Result<&Target>;
}

pub trait TryAsMut<Target: ?Sized> {
    fn try_as_mut(&mut self) -> Result<&mut Target>;
}

impl<T> AsRef<T> for T {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for T {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> TryAsRef<T> for T {
    fn try_as_ref(&self) -> Result<&T> {
        Ok(self)
    }
}

impl<T> TryAsMut<T> for T {
    fn try_as_mut(&mut self) -> Result<&mut T> {
        Ok(self)
    }
}

impl<T: Pod> AsRef<[u8]> for [T] {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T: Pod> TryAsRef<[u8]> for [T] {
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_bytes())
    }
}

impl<T: Pod> AsMut<[u8]> for [T] {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_bytes()
    }
}

impl<T: Pod> TryAsMut<[u8]> for [T] {
    fn try_as_mut(&mut self) -> Result<&mut [u8]> {
        Ok(self.as_mut_bytes())
    }
}

impl<T: Pod> AsRef<[u8]> for T {
    fn as_ref(&self) -> &[u8] {
        mem::as_bytes(self)
    }
}

impl<T: Pod> TryAsRef<[u8]> for T {
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(mem::as_bytes(self))
    }
}

impl<T: Pod> AsMut<[u8]> for T {
    fn as_mut(&mut self) -> &mut [u8] {
        mem::as_mut_bytes(self)
    }
}

impl<T: Pod> TryAsMut<[u8]> for T {
    fn try_as_mut(&mut self) -> Result<&mut [u8]> {
        Ok(mem::as_mut_bytes(self))
    }
}

impl AsRef<[u8]> for str {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl TryAsRef<[u8]> for str {
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_bytes())
    }
}

impl AsRef<[u8]> for char {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_ptr(self as *const _ as *const _, mem::size_of::<char>())
        }
    }
}

impl TryAsRef<[u8]> for char {
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_ref())
    }
}

impl AsRef<[u8]> for [char] {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let ptr = self.as_ptr();
            let size = mem::size_of::<char>() * self.len();
            slice::from_ptr(ptr as *const _, size)
        }
    }
}

impl TryAsRef<[u8]> for [char] {
    fn try_as_ref(&self) -> Result<&[u8]> {
        Ok(self.as_ref())
    }
}
