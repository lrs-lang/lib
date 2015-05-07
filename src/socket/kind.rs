// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_DCCP,
};
use fmt::{Debug, Write};

#[derive(Pod, Eq)]
pub struct Kind(pub c_int);

macro_rules! create {
    ($($name:ident = $val:expr, $doc:expr,)*) => {
        $(#[doc = $doc] pub const $name: Kind = Kind($val);)*

        impl Debug for Kind {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => "Unknown kind",
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    Stream    = SOCK_STREAM,    "Sequenced, reliable, two-way, connection-based byte streams",
    Datagram  = SOCK_DGRAM,     "Unsequenced, unreliable, connection-less messages",
    Raw       = SOCK_RAW,       "Raw network protocol access",
    Rdm       = SOCK_RDM,       "Unsequenced, reliable, connection-less messages",
    SeqPacket = SOCK_SEQPACKET, "Sequenced, reliable, two-way, connection-based messages",
    Dccp      = SOCK_DCCP,      "DCCP",
}
