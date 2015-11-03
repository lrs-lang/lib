// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};

/// Eventfd flags.
#[derive(Pod, Eq)]
pub struct EventfdFlags(pub c_int);

impl BitOr for EventfdFlags {
    type Output = EventfdFlags;
    fn bitor(self, other: EventfdFlags) -> EventfdFlags {
        EventfdFlags(self.0 | other.0)
    }
}

impl BitAnd for EventfdFlags {
    type Output = EventfdFlags;
    fn bitand(self, other: EventfdFlags) -> EventfdFlags {
        EventfdFlags(self.0 & other.0)
    }
}

impl Not for EventfdFlags {
    type Output = EventfdFlags;
    fn not(self) -> EventfdFlags {
        EventfdFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const EFD_NONE: EventfdFlags = EventfdFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: EventfdFlags = EventfdFlags(cty::$val);)*

        impl Debug for EventfdFlags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| cty::$val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first {
                    try!(w.write_all("EFD_NONE".as_bytes()));
                }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Close the eventfd when `exec` is called.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:eventfd(2) and EFD_CLOEXEC therein"]
    flag EFD_CLOSE_ON_EXEC = EFD_CLOEXEC;

    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:eventfd(2) and EFD_NONBLOCK therein"]
    flag EFD_DONT_BLOCK = EFD_NONBLOCK;

    #[doc = "Use semaphore-like behavior.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:eventfd(2) and EFD_NONBLOCK therein"]
    flag EFD_SEMAPHORE = EFD_SEMAPHORE;
}

impl EventfdFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: EventfdFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: EventfdFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: EventfdFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
