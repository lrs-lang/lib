// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_inotify::{Inotify, InodeWatch, InodeData, InodeDataIter};
pub use lrs_inotify::flags::{WatchFlags, InotifyFlags};
pub use lrs_inotify::event::{InodeEvents};

pub mod flags {
    pub use lrs_inotify::flags::{
        WATCH_NONE, INOTIFY_NONE,
        WATCH_DONT_FOLLOW_LINKS, WATCH_NO_UNLINKED, WATCH_OR_EVENTS, WATCH_ONE_SHOT,
        WATCH_ONLY_DIRECTORY, INOTIFY_DONT_BLOCK, INOTIFY_CLOSE_ON_EXEC,
    };
}

pub mod events {
    pub use lrs_inotify::event::{
        INEV_ALL, INEV_CLOSE, INEV_MOVE,
        INEV_NONE, INEV_ACCESS, INEV_MODIFY, INEV_ATTRIB, INEV_CLOSE_WRITE,
        INEV_CLOSE_READ, INEV_OPEN, INEV_MOVED_FROM, INEV_MOVED_TO, INEV_CREATE,
        INEV_DELETE, INEV_DELETE_SELF, INEV_MOVE_SELF, INEV_UNMOUNT, INEV_OVERFLOW,
        INEV_IGNORED, INEV_DIR,
    };
}
