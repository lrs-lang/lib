// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use fmt::{Debug, Write};
use cty::{ST_RDONLY, ST_NOSUID, ST_NODEV, ST_NOEXEC, ST_SYNCHRONOUS, ST_MANDLOCK,
          ST_NOATIME, ST_NODIRATIME, ST_RELATIME, c_ulong};

/// Mount flags of a filesystem.
#[derive(Pod, Eq)]
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
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        let mut first = true;
        let mut add = |s| {
            if !first { try!(w.write(b",").ignore_ok()); }
            first = false;
            w.write(s).ignore_ok()
        };
        if self.read_only()                   { try!(add(b"ReadOnly"))   }
        if self.no_set_id()                   { try!(add(b"NoSetId"))    }
        if self.no_dev()                      { try!(add(b"NoDev"))      }
        if self.no_exec()                     { try!(add(b"NoExec"))     }
        if self.synchronous()                 { try!(add(b"Sync"))       }
        if self.mandatory_locking()           { try!(add(b"ManLock"))    }
        if self.no_access_time_update()       { try!(add(b"NoATime"))    }
        if self.no_dir_access_time_update()   { try!(add(b"NoDirATime")) }
        if self.relative_access_time_update() { try!(add(b"RelATime"))   }
        Ok(())
    }
}
