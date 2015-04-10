// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_int};

pub type FD = c_int;

pub trait FDContainer {
    fn unwrap(self) -> FD;
    fn is_owned(&self) -> bool;
    fn borrow(&self) -> FD;
    fn from_owned(fd: FD) -> Self;
    fn from_borrowed(fd: FD) -> Self;
}
