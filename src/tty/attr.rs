// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use cty::{self, termios2, tcflag_t};
use fmt::{Debug, Write};

#[derive(Pod)]
pub struct TtyAttr(pub termios2);

impl TtyAttr {
    pub fn input_speed(&self) -> u32 {
        self.0.c_ispeed
    }

    pub fn set_input_speed(&mut self, val: u32) {
        self.0.c_ispeed = val;
    }

    pub fn output_speed(&self) -> u32 {
        self.0.c_ospeed
    }

    pub fn set_output_speed(&mut self, val: u32) {
        self.0.c_ospeed = val;
    }
}

impl Debug for TtyAttr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "TtyAttr {{ in: {:?}, out: {:?}, ctrl: {:x} }}",
               TtyInFlags(self.0.c_iflag), TtyOutFlags(self.0.c_oflag), self.0.c_cflag)
    }
}

/// Flags for terminal input.
#[derive(Pod, Eq)]
pub struct TtyInFlags(pub tcflag_t);

impl BitOr for TtyInFlags {
    type Output = TtyInFlags;
    fn bitor(self, other: TtyInFlags) -> TtyInFlags {
        TtyInFlags(self.0 | other.0)
    }
}

impl BitAnd for TtyInFlags {
    type Output = TtyInFlags;
    fn bitand(self, other: TtyInFlags) -> TtyInFlags {
        TtyInFlags(self.0 & other.0)
    }
}

impl Not for TtyInFlags {
    type Output = TtyInFlags;
    fn not(self) -> TtyInFlags {
        TtyInFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const TTYIN_NONE: TtyInFlags = TtyInFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: TtyInFlags = TtyInFlags(cty::$val);)*

        impl Debug for TtyInFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("TTYIN_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Ignore BREAK condition on input.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IGNBRK therein"]
    flag TTYIN_IGNORE_BREAK = IGNBRK;

    #[doc = "BREAK condition causes SIGINT to be sent to the foreground process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and BRKINT therein"]
    flag TTYIN_BREAK_TO_INT = BRKINT;

    #[doc = "Ignore framing and parity errors.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IGNPAR therein"]
    flag TTYIN_IGNORE_ERRORS = IGNPAR;

    #[doc = "Prefix all framing and parity errors with the sequence `0xFF 0x00`.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and PARMRK therein"]
    flag TTYIN_MARK_ERRORS = PARMRK;

    #[doc = "Enable input parity checking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and INPCK therein"]
    flag TTYIN_CHECK_INPUT = INPCK;

    #[doc = "Turn uppercase input into lowercase input.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IUCLC therein"]
    flag TTYIN_TO_LOWER = IUCLC;

    #[doc = "Clear the most significant bit.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and ISTRIP therein"]
    flag TTYIN_TO_ASCII = ISTRIP;

    #[doc = "Transate NL to CR on input.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and INLCR therein"]
    flag TTYIN_NL_TO_CR = ISTRIP;

    #[doc = "Ignore CR input.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IGNCR therein"]
    flag TTYIN_IGNORE_CR = IGNCR;

    #[doc = "Transate CR to NL on input.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and ICRNL therein"]
    flag TTYIN_CR_TO_NL = ICRNL;

    #[doc = "Enable output flow control.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IXON therein"]
    flag TTYIN_OUTPUT_FLOW_CTRL = IXON;

    #[doc = "Enable input flow control.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IXOFF therein"]
    flag TTYIN_INPUT_FLOW_CTRL = IXOFF;

    #[doc = "Typing any character restarts stopped output.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IXANY therein"]
    flag TTYIN_ANY_RESTART = IXANY;

    #[doc = "Input is UTF-8.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and IUTF8 therein"]
    flag TTYIN_UTF8 = IUTF8;
}

/// Flags for terminal output.
#[derive(Pod, Eq)]
pub struct TtyOutFlags(pub tcflag_t);

impl BitOr for TtyOutFlags {
    type Output = TtyOutFlags;
    fn bitor(self, other: TtyOutFlags) -> TtyOutFlags {
        TtyOutFlags(self.0 | other.0)
    }
}

impl BitAnd for TtyOutFlags {
    type Output = TtyOutFlags;
    fn bitand(self, other: TtyOutFlags) -> TtyOutFlags {
        TtyOutFlags(self.0 & other.0)
    }
}

impl Not for TtyOutFlags {
    type Output = TtyOutFlags;
    fn not(self) -> TtyOutFlags {
        TtyOutFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const TTYOUT_NONE: TtyOutFlags = TtyOutFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: TtyOutFlags = TtyOutFlags(cty::$val);)*

        impl Debug for TtyOutFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("TTYOUT_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Enable output processing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and OPOST therein"]
    flag TTYOUT_PROCESS = OPOST;

    #[doc = "Turn lowercase output into uppercase output.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and OLCUC therein"]
    flag TTYOUT_TO_UPPER = OLCUC;

    #[doc = "Transate NL to CRNL on ouput.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and ONLCR therein"]
    flag TTYOUT_NL_TO_CRNL = ONLCR;

    #[doc = "Transate CR to NL on ouput.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and OCRNL therein"]
    flag TTYOUT_CR_TO_NL = OCRNL;

    #[doc = "Don't output CR at column 0.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and ONOCR therein"]
    flag TTYOUT_NO_COL0_CR = ONOCR;

    #[doc = "Don't output CR.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:termios(3) and ONLRET therein"]
    flag TTYOUT_NO_CR = ONLRET;
}
