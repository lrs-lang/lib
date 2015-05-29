// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    IN_ACCESS, IN_ATTRIB, IN_CLOSE_WRITE, IN_CLOSE_NOWRITE, IN_CREATE, IN_DELETE,
    IN_DELETE_SELF, IN_MODIFY, IN_MOVE_SELF, IN_MOVED_FROM, IN_MOVED_TO, IN_OPEN,
    IN_UNMOUNT, IN_ISDIR, IN_IGNORED, IN_Q_OVERFLOW, IN_ALL_EVENTS, IN_MOVE, IN_CLOSE,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

#[repr(C)]
#[derive(Pod, Eq)]
pub struct InodeEvents(pub u32);

impl BitAnd for InodeEvents {
    type Output = InodeEvents;
    fn bitand(self, rhs: InodeEvents) -> InodeEvents { InodeEvents(self.0 & rhs.0) }
}

impl BitOr for InodeEvents {
    type Output = InodeEvents;
    fn bitor(self, rhs: InodeEvents) -> InodeEvents { InodeEvents(self.0 | rhs.0) }
}

impl Not for InodeEvents {
    type Output = InodeEvents;
    fn not(self) -> InodeEvents { InodeEvents(!self.0) }
}

/// Dummy flag with all flags unset.
pub const INEV_NONE: InodeEvents = InodeEvents(0);

/// Shortcut for `INEV_CLOSE_WRITE | INEV_CLOSE_READ`.
pub const INEV_CLOSE: InodeEvents = InodeEvents(IN_CLOSE);

/// Shortcut for `INEV_MOVED_FROM | INEV_MOVED_TO`.
pub const INEV_MOVE: InodeEvents = InodeEvents(IN_MOVE);

/// All events that can be watched for.
pub const INEV_ALL: InodeEvents = InodeEvents(IN_ALL_EVENTS);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: InodeEvents = InodeEvents($val);)*

        impl Debug for InodeEvents {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: u32 = 0 $(| $val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first {
                    try!(w.write_all("INEV_NONE".as_bytes()));
                }
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "The file was accessed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_ACCESS therein"]
    flag INEV_ACCESS = IN_ACCESS;

    #[doc = "The file was modified.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_MODIFY therein"]
    flag INEV_MODIFY = IN_MODIFY;

    #[doc = "Metadata of the file was modified.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_ATTRIB therein"]
    flag INEV_ATTRIB = IN_ATTRIB;

    #[doc = "The file was opened for writing and has been closed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_CLOSE_WRITE therein"]
    flag INEV_CLOSE_WRITE = IN_CLOSE_WRITE;

    #[doc = "The file was opened read-only and has been closed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_CLOSE_NOWRITE therein"]
    flag INEV_CLOSE_READ = IN_CLOSE_NOWRITE;

    #[doc = "The file was opened.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_OPEN therein"]
    flag INEV_OPEN = IN_OPEN;

    #[doc = "A file was moved from this directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_MOVED_FROM therein"]
    flag INEV_MOVED_FROM = IN_MOVED_FROM;

    #[doc = "A file was moved to this directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_MOVED_TO therein"]
    flag INEV_MOVED_TO = IN_MOVED_TO;

    #[doc = "A file was created in this directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_CREATE therein"]
    flag INEV_CREATE = IN_CREATE;

    #[doc = "A file in this directory was deleted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_DELETE therein"]
    flag INEV_DELETE = IN_DELETE;

    #[doc = "The watched file was deleted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_DELETE_SELF therein"]
    flag INEV_DELETE_SELF = IN_DELETE_SELF;

    #[doc = "The watched file was moved.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_MOVE_SELF therein"]
    flag INEV_MOVE_SELF = IN_MOVE_SELF;

    #[doc = "The filesystem containing the file was unmounted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_UNMOUNT therein"]
    flag INEV_UNMOUNT = IN_UNMOUNT;

    #[doc = "The inotify event queue overflowed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_Q_OVERFLOW therein"]
    flag INEV_OVERFLOW = IN_Q_OVERFLOW;

    #[doc = "The watched object has been removed from the inotify object.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_IGNORED therein"]
    flag INEV_IGNORED = IN_IGNORED;

    #[doc = "The object of this event is a directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_ISDIR therein"]
    flag INEV_DIR = IN_ISDIR;
}
