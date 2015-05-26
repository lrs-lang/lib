// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    NLM_F_REQUEST, NLM_F_MULTI, NLM_F_ACK, NLM_F_ECHO, NLM_F_ROOT, NLM_F_ATOMIC,
    NLM_F_REPLACE, NLM_F_EXCL, NLM_F_CREATE, NLM_F_APPEND, NLM_F_MATCH, NLM_F_DUMP,
};
use fmt_::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags set in Netlink messages.
///
/// [field, 1]
/// The integer constant associated with the flags.
///
/// = Remarks
///
/// :flags: link:lrs::netlink::flags
///
/// See {flags} for pre-defined constants.
///
/// = See also
///
/// * {flags}
#[repr(C)]
#[derive(Pod, Eq)]
pub struct NlFlags(pub u16);

impl BitAnd for NlFlags {
    type Output = NlFlags;
    fn bitand(self, rhs: NlFlags) -> NlFlags { NlFlags(self.0 & rhs.0) }
}

impl BitOr for NlFlags {
    type Output = NlFlags;
    fn bitor(self, rhs: NlFlags) -> NlFlags { NlFlags(self.0 | rhs.0) }
}

impl Not for NlFlags {
    type Output = NlFlags;
    fn not(self) -> NlFlags { NlFlags(!self.0) }
}

impl Debug for NlFlags {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "0x{:x}", self.0)
    }
}

pub const NLF_REQUEST : NlFlags = NlFlags(NLM_F_REQUEST);
pub const NLF_MULTI   : NlFlags = NlFlags(NLM_F_MULTI);
pub const NLF_ACK     : NlFlags = NlFlags(NLM_F_ACK);
pub const NLF_ECHO    : NlFlags = NlFlags(NLM_F_ECHO);

pub const NLF_MATCH   : NlFlags = NlFlags(NLM_F_MATCH);
pub const NLF_ROOT    : NlFlags = NlFlags(NLM_F_ROOT);
pub const NLF_ATOMIC  : NlFlags = NlFlags(NLM_F_ATOMIC);
pub const NLF_DUMP    : NlFlags = NlFlags(NLM_F_DUMP);

pub const NLF_REPLACE : NlFlags = NlFlags(NLM_F_REPLACE);
pub const NLF_EXCL    : NlFlags = NlFlags(NLM_F_EXCL);
pub const NLF_CREATE  : NlFlags = NlFlags(NLM_F_CREATE);
pub const NLF_APPEND  : NlFlags = NlFlags(NLM_F_APPEND);
