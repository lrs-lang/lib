// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Types that have a default value.
pub trait Default {
    /// Returns the default value of this type.
    fn default() -> Self;
}

impl Default for () {
    fn default() -> () {
        ()
    }
}
