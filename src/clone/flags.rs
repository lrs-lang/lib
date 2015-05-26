// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    self, c_int,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags that can be used when cloning a process.
///
/// [field, 1]
/// The integer constant associated with the flags.
///
/// = Remarks
///
/// :flags: link:lrs::process::clone
///
/// See {flags} for pre-defined constants.
///
/// = See also
///
/// * {flags}
#[derive(Pod, Eq)]
pub struct CloneFlags(pub c_int);

impl BitAnd for CloneFlags {
    type Output = CloneFlags;
    fn bitand(self, rhs: CloneFlags) -> CloneFlags { CloneFlags(self.0 & rhs.0) }
}

impl BitOr for CloneFlags {
    type Output = CloneFlags;
    fn bitor(self, rhs: CloneFlags) -> CloneFlags { CloneFlags(self.0 | rhs.0) }
}

impl Not for CloneFlags {
    type Output = CloneFlags;
    fn not(self) -> CloneFlags { CloneFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const CLONE_NONE: CloneFlags = CloneFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])*  pub const $name: CloneFlags = CloneFlags(cty::$val);)*

        /// = Remarks
        ///
        /// This prints the flags as a comma-separated list.
        impl Debug for CloneFlags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| cty::$val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & cty::$val != 0 {
                        if !first { try!(w.write(b",")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                let _ = first;
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Share the address space of the process.\n"]
    #[doc = "= Remarks"]
    #[doc = "If this flag is set, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_VM therein"]
    flag CLONE_VM             = CLONE_VM;

    #[doc = "Share the filesystem information of the process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_FS therein"]
    flag CLONE_FS             = CLONE_FS;

    #[doc = "Share the file descriptor table of the process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_FILES therein"]
    flag CLONE_FILES          = CLONE_FILES;

    #[doc = "Clone the address space of the process.\n"]
    #[doc = "= Remarks"]
    #[doc = "If this flag is set, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_SIGHAND therein"]
    flag CLONE_SIGHAND        = CLONE_SIGHAND;

    #[doc = "Trace the cloned process if this process is being traced.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_PTRACE therein"]
    flag CLONE_PTRACE         = CLONE_PTRACE;

    #[doc = "Clone the address space of the process.\n"]
    #[doc = "= Remarks"]
    #[doc = "If this flag is set, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_VM therein"]
    flag CLONE_VFORK          = CLONE_VFORK;

    #[doc = "Set the parent of the child to the parent of the caller.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_PARENT therein"]
    flag CLONE_PARENT         = CLONE_PARENT;

    #[doc = "Clone the address space of the process.\n"]
    #[doc = "= Remarks"]
    #[doc = "If this flag is set, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_VM therein"]
    flag CLONE_THREAD         = CLONE_THREAD;

    #[doc = "Put the child in a new mount namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWNS therein"]
    flag CLONE_NEWMOUNT       = CLONE_NEWNS;

    #[doc = "Share System V semaphores with the process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_SYSVSEM therein"]
    flag CLONE_SYSVSEM        = CLONE_SYSVSEM;

    #[doc = "Set the TLS descriptor of the child.\n"]
    #[doc = "= Remarks"]
    #[doc = "If this flag is set, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_SETTLS therein"]
    flag CLONE_SETTLS         = CLONE_SETTLS;

    #[doc = "Don't allow the child process to be traced.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_UNTRACED therein"]
    flag CLONE_UNTRACED       = CLONE_UNTRACED;

    #[doc = "Put the child in a new UTS namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWUTS therein"]
    flag CLONE_NEWUTS         = CLONE_NEWUTS;

    #[doc = "Put the child in a new IPC namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWIPC therein"]
    flag CLONE_NEWIPC         = CLONE_NEWIPC;

    #[doc = "Put the child in a new user namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWUSER therein"]
    flag CLONE_NEWUSER        = CLONE_NEWUSER;

    #[doc = "Put the child in a new PID namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWPID therein"]
    flag CLONE_NEWPID         = CLONE_NEWPID;

    #[doc = "Put the child in a new network namespace.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_NEWNET therein"]
    flag CLONE_NEWNET         = CLONE_NEWNET;

    #[doc = "Share an I/O context with the process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:clone(2) and CLONE_IO therein"]
    flag CLONE_IO             = CLONE_IO;

    // TODO:
    // flag CLONE_PARENT_SETTID  = CLONE_PARENT_SETTID;
    // flag CLONE_CHILD_CLEARTID = CLONE_CHILD_CLEARTID;
    // flag CLONE_CHILD_SETTID   = CLONE_CHILD_SETTID;
}
