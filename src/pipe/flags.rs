// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Write};
use cty::{
    c_int, c_uint,
    O_CLOEXEC, O_DIRECT, O_NONBLOCK, SPLICE_F_NONBLOCK, SPLICE_F_MORE,
};

/// Pipe flags.
#[derive(Pod, Eq)]
pub struct PipeFlags(pub c_int);

impl BitOr for PipeFlags {
    type Output = PipeFlags;
    fn bitor(self, other: PipeFlags) -> PipeFlags {
        PipeFlags(self.0 | other.0)
    }
}

impl BitAnd for PipeFlags {
    type Output = PipeFlags;
    fn bitand(self, other: PipeFlags) -> PipeFlags {
        PipeFlags(self.0 & other.0)
    }
}

impl Not for PipeFlags {
    type Output = PipeFlags;
    fn not(self) -> PipeFlags {
        PipeFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const PIPE_NONE: PipeFlags = PipeFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: PipeFlags = PipeFlags($val);)*

        impl Debug for PipeFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("PIPE_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Close the pipe when `exec` is called.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:pipe(2) and O_CLOEXEC therein"]
    flag PIPE_CLOSE_ON_EXEC = O_CLOEXEC;

    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:pipe(2) and O_NONBLOCK therein"]
    flag PIPE_DONT_BLOCK = O_NONBLOCK;

    #[doc = "Create a \"datagram\" pipe.\n"]
    #[doc = "= Remarks"]
    #[doc = "== Kernel versions"]
    #[doc = "The minimum required kernel version is 3.4."]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DIRECT therein"]
    flag PIPE_PACKETS = O_DIRECT;
}

impl PipeFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: PipeFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: PipeFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: PipeFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Tee flags.
#[derive(Pod, Eq)]
pub struct TeeFlags(pub c_uint);

impl BitOr for TeeFlags {
    type Output = TeeFlags;
    fn bitor(self, other: TeeFlags) -> TeeFlags {
        TeeFlags(self.0 | other.0)
    }
}

impl BitAnd for TeeFlags {
    type Output = TeeFlags;
    fn bitand(self, other: TeeFlags) -> TeeFlags {
        TeeFlags(self.0 & other.0)
    }
}

impl Not for TeeFlags {
    type Output = TeeFlags;
    fn not(self) -> TeeFlags {
        TeeFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const TEE_NONE: TeeFlags = TeeFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: TeeFlags = TeeFlags($val);)*

        impl Debug for TeeFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("TEE_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:tee(2) and SPLICE_F_NONBLOCK therein"]
    flag TEE_DONT_BLOCK = SPLICE_F_NONBLOCK;
}

impl TeeFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: TeeFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: TeeFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: TeeFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Splice flags.
#[derive(Pod, Eq)]
pub struct SpliceFlags(pub c_uint);

impl BitOr for SpliceFlags {
    type Output = SpliceFlags;
    fn bitor(self, other: SpliceFlags) -> SpliceFlags {
        SpliceFlags(self.0 | other.0)
    }
}

impl BitAnd for SpliceFlags {
    type Output = SpliceFlags;
    fn bitand(self, other: SpliceFlags) -> SpliceFlags {
        SpliceFlags(self.0 & other.0)
    }
}

impl Not for SpliceFlags {
    type Output = SpliceFlags;
    fn not(self) -> SpliceFlags {
        SpliceFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const SPLICE_NONE: SpliceFlags = SpliceFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: SpliceFlags = SpliceFlags($val);)*

        impl Debug for SpliceFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("SPLICE_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:splice(2) and SPLICE_F_NONBLOCK therein"]
    flag SPLICE_DONT_BLOCK = SPLICE_F_NONBLOCK;

    #[doc = "When splicing to a socket, apply MSG_MORE semantics.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:splice(2) and SPLICE_F_MORE therein"]
    #[doc = "* link:man:send(2) and MSG_MORE therein"]
    flag SPLICE_MORE = SPLICE_F_MORE;
}

impl SpliceFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: SpliceFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: SpliceFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: SpliceFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
