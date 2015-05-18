// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use syscall::{getresuid, getresgid, setresuid, setresgid, setgroups, getgroups};
use base::error::{self};
use cty::alias::{UserId, GroupId};
use fmt::{Debug, Write};

/// User ids of a process.
#[derive(Pod, Eq)]
pub struct UserIds {
    /// Real id
    pub real: UserId,
    /// Effective id
    pub effective: UserId,
    /// Saved id
    pub saved: UserId,
}

impl UserIds {
    /// Retrieves the user ids of this process.
    pub fn get() -> UserIds {
        let mut ids = UserIds {
            real:      0,
            effective: 0,
            saved:     0,
        };
        getresuid(&mut ids.real, &mut ids.effective, &mut ids.saved);
        ids
    }

    /// Sets the user ids of this process.
    pub fn set(&self) -> Result {
        rv!(setresuid(self.real, self.effective, self.saved))
    }
}

impl Debug for UserIds {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "UserIds {{ real: {}, effective: {}, saved {} }}",
               self.real, self.effective, self.saved)
    }
}

/// Group ids of a process.
#[derive(Pod, Eq)]
pub struct GroupIds {
    /// Real id
    pub real: GroupId,
    /// Effective id
    pub effective: GroupId,
    /// Saved id
    pub saved: GroupId,
}

impl GroupIds {
    /// Retrieves the group ids of this process.
    pub fn get() -> GroupIds {
        let mut ids = GroupIds {
            real:      0,
            effective: 0,
            saved:     0,
        };
        getresgid(&mut ids.real, &mut ids.effective, &mut ids.saved);
        ids
    }

    /// Sets the group ids of this process.
    pub fn set(&self) -> Result {
        rv!(setresgid(self.real, self.effective, self.saved))
    }
}

impl Debug for GroupIds {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "GroupIds {{ real: {}, effective: {}, saved {} }}",
               self.real, self.effective, self.saved)
    }
}

/// Sets all user ids to the real id.
pub fn drop_user_privileges() -> Result {
    let mut ids = UserIds::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets all group ids to the real id.
pub fn drop_group_privileges() -> Result {
    let mut ids = GroupIds::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets the effective user id of this process.
///
/// [argument, id]
/// The new effective user id of the process.
pub fn set_effective_user_id(id: UserId) -> Result {
    rv!(setresuid(-1, id, -1))
}

/// Sets the effective group id.
///
/// [argument, id]
/// The new effective group id of the process.
pub fn set_effective_group_id(id: GroupId) -> Result {
    rv!(setresgid(-1, id, -1))
}

/// Returns the number of supplementary groups.
pub fn num_supplementary_groups() -> usize {
    getgroups(&mut []) as usize
}

const MAX_SUP_GROUPS: usize = 65536;

/// Retrieves the supplementary groups.
///
/// [argument, buf]
/// The buffer in which the supplementary groups will be stored.
///
/// [return_value]
/// Returns the number of supplementary groups stored
pub fn supplementary_groups(buf: &mut [GroupId]) -> Result<usize> {
    if buf.len() > MAX_SUP_GROUPS {
        rv!(getgroups(&mut buf[..MAX_SUP_GROUPS]), -> usize)
    } else {
        rv!(getgroups(buf), -> usize)
    }
}

/// Sets the supplementary groups.
///
/// [argument, buf]
/// The buffer which contains the new supplementary groups.
pub fn set_supplementary_groups(buf: &[GroupId]) -> Result {
    if buf.len() > MAX_SUP_GROUPS {
        return Err(error::InvalidArgument);
    }
    rv!(setgroups(buf))
}
