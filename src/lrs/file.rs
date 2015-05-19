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
pub use lrs_file::flags::{FileFlags, Mode, AccessMode};
pub use lrs_file::info::{Info, Type};
pub use lrs_dev::{Device, DeviceType};

/// File flag constants.
pub mod flags {
    pub use lrs_file::flags::{
        FILE_CREATE, FILE_READ_ONLY, FILE_WRITE_ONLY, FILE_READ_WRITE, FILE_CLOSE_ON_EXEC,
        FILE_BYPASS_BUFFER, FILE_ONLY_DIRECTORY, FILE_EXCLUSIVE,
        FILE_NO_ACCESS_TIME_UPDATE, FILE_NO_CONTROLLING_TERM, FILE_DONT_FOLLOW_LINKS,
        FILE_TRUNCATE, FILE_APPEND, FILE_SIGNAL_IO, FILE_SYNC, FILE_DATA_SYNC,
        FILE_DONT_BLOCK, FILE_PATH, FILE_TEMP, FILE_LARGE,
    };
}

/// File mode constants.
pub mod mode {
    pub use lrs_file::flags::{
        MODE_FILE, MODE_DIRECTORY, MODE_SET_USER_ID, MODE_SET_GROUP_ID, MODE_STICKY,
        MODE_USER_READ, MODE_USER_WRITE, MODE_USER_EXEC, MODE_GROUP_READ,
        MODE_GROUP_WRITE, MODE_GROUP_EXEC, MODE_WORLD_READ, MODE_WORLD_WRITE,
        MODE_WORLD_EXEC,
    };
}
