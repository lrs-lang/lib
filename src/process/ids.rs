use libc::{getresuid, getresgid, setresuid, setresgid, setgroups, getgroups, uid_t, c_int,
           gid_t, size_t};
use result::{Result};
use errno::{self};

/// User ids of a process.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub struct User {
    /// Real id
    pub real:      uid_t,
    /// Effective id
    pub effective: uid_t,
    /// Saved id
    pub saved:     uid_t,
}

impl User {
    /// Retrieves the user ids of this process.
    pub fn get() -> User {
        let mut ids = User {
            real:      0,
            effective: 0,
            saved:     0,
        };
        unsafe { getresuid(&mut ids.real, &mut ids.effective, &mut ids.saved); }
        ids
    }

    /// Sets the user ids of this process.
    pub fn set(&self) -> Result<()> {
        match unsafe { setresuid(self.real, self.effective, self.saved) } {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }
}

/// Group ids of a process.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub struct Group {
    /// Real id
    pub real:      gid_t,
    /// Effective id
    pub effective: gid_t,
    /// Saved id
    pub saved:     gid_t,
}

impl Group {
    /// Retrieves the group ids of this process.
    pub fn get() -> Group {
        let mut ids = Group {
            real:      0,
            effective: 0,
            saved:     0,
        };
        unsafe { getresgid(&mut ids.real, &mut ids.effective, &mut ids.saved); }
        ids
    }

    /// Sets the group ids of this process.
    pub fn set(&self) -> Result<()> {
        match unsafe { setresgid(self.real, self.effective, self.saved) } {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }
}

/// Sets all user ids to the real id.
pub fn user_drop_privileges() -> Result<()> {
    let mut ids = User::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets all group ids to the real id.
pub fn group_drop_privileges() -> Result<()> {
    let mut ids = Group::get();
    ids.effective = ids.real;
    ids.saved     = ids.real;
    ids.set()
}

/// Sets the effective user id.
pub fn user_set_effective(id: uid_t) -> Result<()> {
    match unsafe { setresuid(-1, id, -1) } {
        -1 => Err(errno::get()),
        _ => Ok(()),
    }
}

/// Sets the effective group id.
pub fn group_set_effective(id: gid_t) -> Result<()> {
    match unsafe { setresgid(-1, id, -1) } {
        -1 => Err(errno::get()),
        _ => Ok(()),
    }
}

/// Gets the number of supplementary groups.
pub fn num_supplementary_groups() -> usize {
    unsafe { getgroups(0, 0 as *mut _) as usize }
}

/// Retreives the supplementary groups.
pub fn supplementary_groups(buf: &mut [gid_t]) -> Result<usize> {
    if buf.len() > 65536 {
        return Err(errno::InvalidArgument);
    }
    match unsafe { getgroups(buf.len() as c_int, buf.as_mut_ptr()) } {
        -1 => Err(errno::get()),
        n => Ok(n as usize),
    }
}

/// Sets the supplementary groups.
pub fn set_supplementary_groups(buf: &[gid_t]) -> Result<()> {
    if buf.len() > 65536 {
        return Err(errno::InvalidArgument);
    }
    match unsafe { setgroups(buf.len() as size_t, buf.as_ptr()) } {
        -1 => Err(errno::get()),
        _ => Ok(()),
    }
}
