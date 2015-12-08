// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Raw file descriptor handling.

pub use lrs_fd::{STDIN, STDOUT, STDERR, FdIo, FdContainer};
pub use lrs_fd::flags::{DescriptionFlags};

/// File description flags.
pub mod flags {
    pub use lrs_fd::flags::{
        FD_ACCESS_MASK,
        FD_NONE, FD_READ_ONLY, FD_WRITE_ONLY, FD_READ_WRITE, FD_BYPASS_BUFFER,
        FD_NO_ACCESS_TIME_UPDATE, FD_APPEND, FD_SIGNAL_IO, FD_SYNC, FD_DATA_SYNC,
        FD_DONT_BLOCK, FD_PATH,
    };
}
