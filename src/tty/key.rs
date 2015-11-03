// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

use base::prelude::*;
use fmt::{Debug, Write};
use cty::{self};

#[derive(Pod, Eq)]
pub struct TtyKey(pub usize);

macro_rules! create {
    ($($(#[$meta:meta])* key $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: TtyKey = TtyKey(cty::$val as usize);)*

        impl Debug for TtyKey {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Unknown({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    key Interrupt   = VINTR;
    key Quit        = VQUIT;
    key EraseChar   = VERASE;
    key EraseLine   = VKILL;
    key EndOfFile   = VEOF;
    key Timeout     = VTIME;
    key MinInput    = VMIN;
    key StartOutput = VSTART;
    key StopOutput  = VSTOP;
    key Suspend     = VSUSP;
    key Reprint     = VREPRINT;
    key EraseWord   = VWERASE;
    key Escape      = VLNEXT;
    key EndOfLine   = VEOL;
    key EndOfLine2  = VEOL2;
}
