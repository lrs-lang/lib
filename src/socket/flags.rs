// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use cty::{
    c_int, SOCK_NONBLOCK, SOCK_CLOEXEC,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags that can be used when opening a socket.
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
pub struct SockFlags(pub c_int);

impl BitAnd for SockFlags {
    type Output = SockFlags;
    fn bitand(self, rhs: SockFlags) -> SockFlags { SockFlags(self.0 & rhs.0) }
}

impl BitOr for SockFlags {
    type Output = SockFlags;
    fn bitor(self, rhs: SockFlags) -> SockFlags { SockFlags(self.0 | rhs.0) }
}

impl Not for SockFlags {
    type Output = SockFlags;
    fn not(self) -> SockFlags { SockFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const SOCK_NONE: SockFlags = SockFlags(0);


macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: SockFlags = SockFlags($val);)*

        impl Debug for SockFlags {
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
                let _ = first;
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Sets the file descriptor to non-blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:socket(2) and SOCK_NONBLOCK therein"]
    flag SOCK_DONT_BLOCK = SOCK_NONBLOCK;

    #[doc = "Sets the close-on-exec flag on the file descriptor.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:socket(2) and SOCK_CLOEXEC therein"]
    flag SOCK_CLOSE_ON_EXEC = SOCK_CLOEXEC;
}
