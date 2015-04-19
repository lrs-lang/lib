// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::syscall::{getresuid, getresgid, setresuid, setresgid, setgroups, getgroups};
use base::result::{Result};
use base::error::{self};
use base::alias::{UserId, GroupId};

/// User ids of a process.
#[derive(Copy, Eq)]
pub struct UserIds {
    /// Real id
    pub real:      UserId,
    /// Effective id
    pub effective: UserId,
    /// Saved id
    pub saved:     UserId,
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

/// Group ids of a process.
#[derive(Copy, Eq)]
pub struct GroupIds {
    /// Real id
    pub real:      GroupId,
    /// Effective id
    pub effective: GroupId,
    /// Saved id
    pub saved:     GroupId,
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

/// Sets all user ids to the real id.
pub fn user_drop_privileges() -> Result {
    let mut ids = UserIds::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets all group ids to the real id.
pub fn group_drop_privileges() -> Result {
    let mut ids = GroupIds::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets the effective user id.
pub fn user_set_effective_ids(id: UserId) -> Result {
    rv!(setresuid(-1, id, -1))
}

/// Sets the effective group id.
pub fn group_set_effective_ids(id: GroupId) -> Result {
    rv!(setresgid(-1, id, -1))
}

/// Gets the number of supplementary groups.
pub fn num_supplementary_groups() -> usize {
    getgroups(&mut []) as usize
}

/// Retreives the supplementary groups.
pub fn supplementary_groups(buf: &mut [GroupId]) -> Result<usize> {
    if buf.len() > 65536 {
        return Err(error::InvalidArgument);
    }
    rv!(getgroups(buf), -> usize)
}

/// Sets the supplementary groups.
pub fn set_supplementary_groups(buf: &[GroupId]) -> Result {
    if buf.len() > 65536 {
        return Err(error::InvalidArgument);
    }
    rv!(setgroups(buf))
}
