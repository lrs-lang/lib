// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Write};
use cty::{
    c_int,
    O_APPEND, O_NONBLOCK, O_DIRECT, O_NOATIME, O_ASYNC, O_SYNC, O_DSYNC, O_PATH, O_RDWR,
    O_RDONLY, O_WRONLY,
};

/// File description flags.
#[derive(Pod, Eq)]
pub struct DescriptionFlags(pub c_int);

impl BitOr for DescriptionFlags {
    type Output = DescriptionFlags;
    fn bitor(self, other: DescriptionFlags) -> DescriptionFlags {
        DescriptionFlags(self.0 | other.0)
    }
}

impl BitAnd for DescriptionFlags {
    type Output = DescriptionFlags;
    fn bitand(self, other: DescriptionFlags) -> DescriptionFlags {
        DescriptionFlags(self.0 & other.0)
    }
}

impl Not for DescriptionFlags {
    type Output = DescriptionFlags;
    fn not(self) -> DescriptionFlags {
        DescriptionFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const FD_NONE: DescriptionFlags = DescriptionFlags(0);

/// Mask containing the access flags.
///
/// = Remarks
///
/// That is, FD_READ_ONLY, FD_WRITE_ONLY, and FD_READ_WRITE.
pub const FD_ACCESS_MASK: DescriptionFlags = DescriptionFlags(3);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: DescriptionFlags = DescriptionFlags($val);)*

        impl Debug for DescriptionFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let rm = match self.0 & 3 {
                    O_RDONLY => "FD_READ_ONLY",
                    O_WRONLY => "FD_WRITE_ONLY",
                    _ => "FD_READ_WRITE",
                };
                try!(w.write_all(rm.as_bytes()));
                let flags = self.0 & !3;
                $(
                    if flags & $val != 0 {
                        try!(w.write(b"|"));
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "The file description is in read-only mode.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_RDONLY therein"]
    flag FD_READ_ONLY = O_RDONLY;

    #[doc = "The file description is in write-only mode.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_WRONLY therein"]
    flag FD_WRITE_ONLY = O_WRONLY;

    #[doc = "The file description is open for reading and writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_RDWR therein"]
    flag FD_READ_WRITE = O_RDWR;

    #[doc = "Bypass kernel buffers and write directly to the disk.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DIRECT therein"]
    flag FD_BYPASS_BUFFER = O_DIRECT;

    #[doc = "Don't update the access time of the file.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NOATIME therein"]
    flag FD_NO_ACCESS_TIME_UPDATE = O_NOATIME;

    #[doc = "Perform all writes to the file at the end of the file.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_APPEND therein"]
    flag FD_APPEND = O_APPEND;

    #[doc = "Send a signal to the process when the file becomes ready for reading or \
             writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_ASYNC therein"]
    flag FD_SIGNAL_IO = O_ASYNC;

    #[doc = "Ensure that all data has been passed to the hardware after a write.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_SYNC therein"]
    flag FD_SYNC = O_SYNC;

    #[doc = "Ensure that enough data has been passed to the hardware after a write so \
             that the data can be read back.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DSYNC therein"]
    flag FD_DATA_SYNC = O_DSYNC;

    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NONBLOCK therein"]
    flag FD_DONT_BLOCK = O_NONBLOCK;

    #[doc = "The file description identifies a path in a filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_PATH therein"]
    flag FD_PATH = O_PATH;
}

impl DescriptionFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: DescriptionFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: DescriptionFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: DescriptionFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
