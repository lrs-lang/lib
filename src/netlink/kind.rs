// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    NLMSG_NOOP, NLMSG_ERROR, NLMSG_DONE,
};
use fmt_::{Debug, Write};

/// A Netlink message type.
///
/// [field, 1]
/// The integer constant associated with the  type.
///
/// = Remarks
///
/// :kinds: link:lrs::netlink::kind
///
/// See {kinds} for pre-defined constants.
///
/// = See also
///
/// * {kinds}
#[repr(C)]
#[derive(Pod, Eq)]
pub struct Kind(pub u16);

impl Debug for Kind {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        match *self {
            NoOp     => w.write_all(b"NoOp").ignore_ok(),
            ErrorAck => w.write_all(b"ErrorAck").ignore_ok(),
            Done     => w.write_all(b"Done").ignore_ok(),
            _ => write!(w, "{}", self.0),
        }
    }
}

pub const NoOp                  : Kind = Kind(NLMSG_NOOP);
pub const ErrorAck              : Kind = Kind(NLMSG_ERROR);
pub const Done                  : Kind = Kind(NLMSG_DONE);
