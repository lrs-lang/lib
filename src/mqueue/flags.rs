// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Write};
use cty::{
    self, k_long,
};

/// Flags for opening and modifying a message queue.
#[derive(Pod, Eq)]
pub struct MqFlags(pub k_long);

impl BitOr for MqFlags {
    type Output = MqFlags;
    fn bitor(self, other: MqFlags) -> MqFlags {
        MqFlags(self.0 | other.0)
    }
}

impl BitAnd for MqFlags {
    type Output = MqFlags;
    fn bitand(self, other: MqFlags) -> MqFlags {
        MqFlags(self.0 & other.0)
    }
}

impl Not for MqFlags {
    type Output = MqFlags;
    fn not(self) -> MqFlags {
        MqFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const MQ_NONE: MqFlags = MqFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MqFlags = MqFlags(cty::$val as k_long);)*

        impl Debug for MqFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val as k_long != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("MQ_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Don't block when sending and receiving on the queue.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mq_getattr(3) and O_NONBLOCK therein"]
    flag MQ_DONT_BLOCK = O_NONBLOCK;
}
