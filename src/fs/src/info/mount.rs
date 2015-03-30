// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::{Debug, Formatter, Error};

use core::cty::{ST_RDONLY, ST_NOSUID, ST_NODEV, ST_NOEXEC, ST_SYNCHRONOUS, ST_MANDLOCK,
                ST_NOATIME, ST_NODIRATIME, ST_RELATIME, c_ulong};

/// Mount flags of a filesystem.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags(pub c_ulong);

impl Flags {
    /// Read only
    pub fn read_only(self)                   -> bool { self.0 & ST_RDONLY      != 0 }
    /// No set-user-ID / set-group-ID
    pub fn no_set_id(self)                   -> bool { self.0 & ST_NOSUID      != 0 }
    /// No access to device special files.
    pub fn no_dev(self)                      -> bool { self.0 & ST_NODEV       != 0 }
    /// No execution.
    pub fn no_exec(self)                     -> bool { self.0 & ST_NOEXEC      != 0 }
    /// Data and metadata is written to disk immediately.
    pub fn synchronous(self)                 -> bool { self.0 & ST_SYNCHRONOUS != 0 }
    /// Mandatory locking is enabled.
    pub fn mandatory_locking(self)           -> bool { self.0 & ST_MANDLOCK    != 0 }
    /// Access time is not automatically updated.
    pub fn no_access_time_update(self)       -> bool { self.0 & ST_NOATIME     != 0 }
    /// Access time is not automatically updated for directorise.
    pub fn no_dir_access_time_update(self)   -> bool { self.0 & ST_NODIRATIME  != 0 }
    /// Access time is updated relative to creation and modification time.
    pub fn relative_access_time_update(self) -> bool { self.0 & ST_RELATIME    != 0 }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut first = true;
        let mut add = |s| {
            if !first { try!(f.write_str(",")); }
            first = false;
            f.write_str(s)
        };
        if self.read_only()                   { try!(add("ReadOnly"))   }
        if self.no_set_id()                   { try!(add("NoSetId"))    }
        if self.no_dev()                      { try!(add("NoDev"))      }
        if self.no_exec()                     { try!(add("NoExec"))     }
        if self.synchronous()                 { try!(add("Sync"))       }
        if self.mandatory_locking()           { try!(add("ManLock"))    }
        if self.no_access_time_update()       { try!(add("NoATime"))    }
        if self.no_dir_access_time_update()   { try!(add("NoDirATime")) }
        if self.relative_access_time_update() { try!(add("RelATime"))   }
        Ok(())
    }
}
