// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File handling.

pub use lrs_file::{File, Seek, Advice, info_no_follow, exists, can_access, set_len,
                     link, TimeChange, set_times, set_times_no_follow, exchange, rename,
                     create_dir, remove, symlink, read_link_buf, read_link, change_owner,
                     change_owner_no_follow, change_mode, create_file, create_device,
                     set_attr, set_attr_no_follow, get_attr_buf, get_attr_no_follow_buf,
                     get_attr, get_attr_no_follow, remove_attr, remove_attr_no_follow,
                     list_attr_size, list_attr_size_no_follow, list_attr_buf,
                     list_attr_buf_no_follow, list_attr, list_attr_no_follow};
pub use lrs_file::_info as info;
pub use lrs_file::flags::{Flags, Mode, AccessMode};
pub use lrs_file::info::{Info, Type};
pub use lrs_dev::{Device, DeviceType};
