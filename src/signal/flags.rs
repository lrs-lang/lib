// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


#[prelude_import] use base::prelude::*;
use cty::{ self,
    c_int,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags for changing a signal handler.
#[derive(Pod, Eq)]
pub struct SigFlags(pub c_int);

impl BitAnd for SigFlags {
    type Output = SigFlags;
    fn bitand(self, rhs: SigFlags) -> SigFlags { SigFlags(self.0 & rhs.0) }
}

impl BitOr for SigFlags {
    type Output = SigFlags;
    fn bitor(self, rhs: SigFlags) -> SigFlags { SigFlags(self.0 | rhs.0) }
}

impl Not for SigFlags {
    type Output = SigFlags;
    fn not(self) -> SigFlags { SigFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const SA_NONE: SigFlags = SigFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])*  pub const $name: SigFlags = SigFlags(cty::$val);)*

        /// = Remarks
        ///
        /// This prints the flags as a comma-separated list.
        impl Debug for SigFlags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| cty::$val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & cty::$val != 0 {
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
    #[doc = "Don't send notifications when a child process stops or resumes.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_NOCLDSTOP therein"]
    flag SA_NOCLDSTOP = SA_NOCLDSTOP;

    #[doc = "Don't turn dead children into zombies.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_NOCLDWAIT therein"]
    flag SA_NOCLDWAIT = SA_NOCLDWAIT;

    // #[doc = "Don't send notifications when a child process stops or resumes.\n"]
    // #[doc = "= See also"]
    // #[doc = "* link:man:sigaction(2) and SA_SIGINFO therein"]
    // flag SA_SIGINFO   = SA_SIGINFO;

    #[doc = "Use the alternative stack for this signal handler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_ONSTACK therein"]
    flag SA_ALT_STACK   = SA_ONSTACK;

    #[doc = "Restart syscalls after handling this signal.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_RESTART therein"]
    flag SA_RESTART   = SA_RESTART;

    #[doc = "Don't block the signal while it's being handled.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_NODEFER therein"]
    flag SA_NODEFER   = SA_NODEFER;

    #[doc = "Reset the handler to the default upon entry to the signal handler.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sigaction(2) and SA_RESETHAND therein"]
    flag SA_RESETHAND = SA_RESETHAND;
}
