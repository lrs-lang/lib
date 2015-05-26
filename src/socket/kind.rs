// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_DCCP,
};
use fmt::{Debug, Write};

/// A socket type.
///
/// [field, 1]
/// The integer constant associated with the socket type.
///
/// = Remarks
///
/// :kinds: link:lrs::socket::kind
///
/// See {kinds} for pre-defined constants.
///
/// = See also
///
/// * {kinds}
#[derive(Pod, Eq)]
pub struct Kind(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* kind $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: Kind = Kind($val);)*

        impl Debug for Kind {
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
    #[doc = "Sequenced, reliable, two-way, connection-based byte streams"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_STREAM therein"]
    kind Stream = SOCK_STREAM;

    #[doc = "Unsequenced, unreliable, connection-less messages"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_DGRAM therein"]
    kind Datagram = SOCK_DGRAM;

    #[doc = "Raw network protocol access"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_RAW therein"]
    kind Raw = SOCK_RAW;

    #[doc = "Unsequenced, reliable, connection-less messages"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_RDM therein"]
    kind Rdm = SOCK_RDM;

    #[doc = "Sequenced, reliable, two-way, connection-based messages"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_SEQPACKET therein"]
    kind SeqPacket = SOCK_SEQPACKET;

    #[doc = "DCCP"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:socket(2) and SOCK_DCCP therein"]
    kind Dccp = SOCK_DCCP;
}
