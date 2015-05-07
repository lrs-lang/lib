// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    MSG_OOB, MSG_PEEK, MSG_DONTROUTE, MSG_CTRUNC, MSG_PROBE, MSG_TRUNC, MSG_DONTWAIT,
    MSG_EOR, MSG_WAITALL, MSG_CONFIRM, MSG_ERRQUEUE, MSG_MORE, MSG_WAITFORONE,
    MSG_FASTOPEN, MSG_NOSIGNAL, MSG_CMSG_CLOEXEC,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Per-message flags
///
/// See the module for more details and pre-defined constants.
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
    ($($name:ident = $val:expr, $doc:expr,)*) => {
        $(#[doc = $doc] pub const $name: Flags = Flags($val);)*

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
    None         = 0                , "No flags",
    Confirm      = MSG_CONFIRM      , "See the module documentation",
    DontRoute    = MSG_DONTROUTE    , "See the module documentation",
    DontBlock    = MSG_DONTWAIT     , "See the module documentation",
    EndOfRecord  = MSG_EOR          , "See the module documentation",
    More         = MSG_MORE         , "See the module documentation",
    OutOfBand    = MSG_OOB          , "See the module documentation",
    ErrorQueue   = MSG_ERRQUEUE     , "See the module documentation",
    Peek         = MSG_PEEK         , "See the module documentation",
    RealSize     = MSG_TRUNC        , "See the module documentation",
    WaitAll      = MSG_WAITALL      , "See the module documentation",
    WaitForOne   = MSG_WAITFORONE   , "See the module documentation",
    CMsgRealSize = MSG_CTRUNC       , "See the module documentation",
    Probe        = MSG_PROBE        , "See the module documentation",
    FastOpen     = MSG_FASTOPEN     , "See the module documentation",
    NoSignal     = MSG_NOSIGNAL     , "See the module documentation",
    CMsgCloexec  = MSG_CMSG_CLOEXEC , "See the module documentation",
}
