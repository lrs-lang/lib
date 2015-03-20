// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use imp::syscall::{sync};

pub mod info;

/// Writes all buffered data and metadata to the disks.
pub fn sync_all() {
    sync()
}
