// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int, SOCK_NONBLOCK, SOCK_CLOEXEC,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags that can be used when opering a socket.
///
/// [field, 1]
/// The integer constant associated with the flags.
///
/// = Remarks
///
/// :flags: link:lrs::socket::flags
///
/// See {flags} for pre-defined constants.
///
/// = See also
///
/// * {flags}
#[derive(Pod, Eq)]
pub struct Flags(pub c_int);

impl BitAnd for Flags {
    type Output = Flags;
    fn bitand(self, rhs: Flags) -> Flags { Flags(self.0 & rhs.0) }
}

impl BitOr for Flags {
    type Output = Flags;
    fn bitor(self, rhs: Flags) -> Flags { Flags(self.0 | rhs.0) }
}

impl Not for Flags {
    type Output = Flags;
    fn not(self) -> Flags { Flags(!self.0) }
}

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: Flags = Flags($val);)*

        /// = Remarks
        ///
        /// This prints the flags as a comma-separated list.
        impl Debug for Flags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| $val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & $val != 0 {
                        if !first { try!(w.write(b",")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                let _ = first;
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "No flags"]
    flag None = 0;

    #[doc = "Sets the file descriptor to non-blocking"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_NONBLOCK therein"]
    flag NonBlocking = SOCK_NONBLOCK;

    #[doc = "Sets the close-on-exec flag on the file descriptor"]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This flag will always be set."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_CLOEXEC therein"]
    flag CloseOnExec = SOCK_CLOEXEC;
}
