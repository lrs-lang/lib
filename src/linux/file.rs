// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File handling.

pub use linux_file::{File, Seek, Advice, info_no_follow, exists, can_access, set_len,
                     link, Time, set_times, set_times_no_follow, exchange, rename, mkdir,
                     remove, symlink, read_link_buf, read_link};
pub use linux_file::_info as info;
pub use linux_file::flags::{Flags, Mode, AccessMode};
pub use linux_file::info::{Info, Type};
pub use linux_dev::{Device, DeviceType};
