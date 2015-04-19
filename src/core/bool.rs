// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};

impl Eq for bool {
    fn eq(&self, other: &bool) -> bool {
        *self == *other
    }
}
