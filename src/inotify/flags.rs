// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    IN_EXCL_UNLINK, IN_MASK_ADD, IN_ONESHOT, IN_ONLYDIR, IN_DONT_FOLLOW,
    IN_NONBLOCK, IN_CLOEXEC,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

#[derive(Pod, Eq)]
pub struct WatchFlags(pub u32);

impl BitAnd for WatchFlags {
    type Output = WatchFlags;
    fn bitand(self, rhs: WatchFlags) -> WatchFlags { WatchFlags(self.0 & rhs.0) }
}

impl BitOr for WatchFlags {
    type Output = WatchFlags;
    fn bitor(self, rhs: WatchFlags) -> WatchFlags { WatchFlags(self.0 | rhs.0) }
}

impl Not for WatchFlags {
    type Output = WatchFlags;
    fn not(self) -> WatchFlags { WatchFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const WATCH_NONE: WatchFlags = WatchFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: WatchFlags = WatchFlags($val);)*

        impl Debug for WatchFlags {
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
                    try!(w.write_all("WATCH_NONE".as_bytes()));
                }
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Don't follow symlinks when adding the watch.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_DONT_FOLLOW therein"]
    flag WATCH_DONT_FOLLOW_LINKS = IN_DONT_FOLLOW;

    #[doc = "Don't generate events for unlinked children.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_EXCL_UNLINK therein"]
    flag WATCH_NO_UNLINKED = IN_EXCL_UNLINK;

    #[doc = "If a watch already exists, extend the watched events by OR-ing the masks.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_MASK_ADD therein"]
    flag WATCH_OR_EVENTS = IN_MASK_ADD;

    #[doc = "Remove the watch after the first event has been generated.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_ONESHOT therein"]
    flag WATCH_ONE_SHOT = IN_ONESHOT;

    #[doc = "Only add the watch if the path refers to a directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify(7) and IN_ONLYDIR therein"]
    flag WATCH_ONLY_DIRECTORY = IN_ONLYDIR;
}


#[derive(Pod, Eq)]
pub struct InotifyFlags(pub c_int);

impl BitAnd for InotifyFlags {
    type Output = InotifyFlags;
    fn bitand(self, rhs: InotifyFlags) -> InotifyFlags { InotifyFlags(self.0 & rhs.0) }
}

impl BitOr for InotifyFlags {
    type Output = InotifyFlags;
    fn bitor(self, rhs: InotifyFlags) -> InotifyFlags { InotifyFlags(self.0 | rhs.0) }
}

impl Not for InotifyFlags {
    type Output = InotifyFlags;
    fn not(self) -> InotifyFlags { InotifyFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const INOTIFY_NONE: InotifyFlags = InotifyFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: InotifyFlags = InotifyFlags($val);)*

        impl Debug for InotifyFlags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| $val)*;
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
                    try!(w.write_all("INOTIFY_NONE".as_bytes()));
                }
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Don't block when reading from the returned file descriptor.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify_init1(2) and IN_NONBLOCK therein"]
    flag INOTIFY_DONT_BLOCK = IN_NONBLOCK;

    #[doc = "Close the file descriptor when `exec` is called.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:inotify_init1(2) and IN_CLOEXEC therein"]
    flag INOTIFY_CLOSE_ON_EXEC = IN_CLOEXEC;
}
