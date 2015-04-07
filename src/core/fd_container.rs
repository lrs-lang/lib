// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_int};

pub trait FDContainer {
    fn unwrap(self) -> c_int;
    fn is_owned(&self) -> bool;
    fn borrow(&self) -> c_int;
    fn from_owned(fd: c_int) -> Self;
    fn from_borrowed(fd: c_int) -> Self;
}
