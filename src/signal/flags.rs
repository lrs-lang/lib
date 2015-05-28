// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


#[prelude_import] use base::prelude::*;
use cty::{ self,
    c_int,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

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
    flag SA_NOCLDSTOP = SA_NOCLDSTOP;
    flag SA_NOCLDWAIT = SA_NOCLDWAIT;
    flag SA_SIGINFO   = SA_SIGINFO;
    flag SA_ONSTACK   = SA_ONSTACK;
    flag SA_RESTART   = SA_RESTART;
    flag SA_NODEFER   = SA_NODEFER;
    flag SA_RESETHAND = SA_RESETHAND;
}
