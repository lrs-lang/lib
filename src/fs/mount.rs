// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::ops::{BitOr, Not, BitAnd};
use cty::{c_ulong, MS_RDONLY, MS_NOSUID, MS_NODEV, MS_NOEXEC, MS_SYNCHRONOUS,
          MS_REMOUNT, MS_MANDLOCK, MS_DIRSYNC, MS_NOATIME, MS_NODIRATIME,
          MS_BIND, MS_MOVE, MS_REC, MS_SILENT, MS_POSIXACL, MS_UNBINDABLE, MS_LAZYTIME,
          MS_PRIVATE, MS_SLAVE, MS_SHARED, MS_STRICTATIME, PATH_MAX};
use fmt::{Debug, Write};
use syscall::{self};
use rmo::{ToRmo};
use str_one::{CStr};
use str_two::{CString};

use {rmo_cstr, Pool};

/// Mounts a filesystem.
///
/// [argument, src]
/// The file that will be mounted.
///
/// [argument, dst]
/// The point at which it will be mounted.
///
/// [argument, ty]
/// The type of the filesystem.
///
/// [argument, flags]
/// The flags to be used to mount the filesystem.
///
/// [argument, data]
/// Filesystem dependent data.
///
/// = Remarks
///
/// :flags: link:lrs::fs::flags
///
/// See {flags} for pre-defined mount flags.
///
/// = Examples
///
/// The following example bind-mounts a directory `a` read-only at the path `b`. Both
/// paths must exist in the current working directory and the example must be executed as
/// root.
///
/// ----
/// mount("a", "b", "", MOUNT_READ_ONLY | MOUNT_BIND, "").unwrap();
/// ----
///
/// The example in link:lrs::fs::unmount[unmount] shows how to perform the unmount
/// operation.
///
/// = See also
///
/// * link:man:mount(2)
/// * {flags}
pub fn mount<P, Q, R, S>(src: P, dst: Q, ty: R, flags: MountFlags, data: S) -> Result
    where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          Q: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          R: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
          S: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
{
    let mut buf1: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf2: [d8; PATH_MAX] = unsafe { mem::uninit() };
    let mut buf3: [d8; 256] = unsafe { mem::uninit() };
    let mut buf4: [d8; 256] = unsafe { mem::uninit() };
    let src  = try!(rmo_cstr(&src,  &mut buf1));
    let dst  = try!(rmo_cstr(&dst,  &mut buf2));
    let ty   = try!(rmo_cstr(&ty,   &mut buf3));
    let data = try!(rmo_cstr(&data, &mut buf4));
    rv!(syscall::mount(&src, &dst, &ty, flags.0, &data))
}

/// Flags used when mounting a filesystem.
///
/// = Remarks
///
/// :flags: link:lrs::fs::flags
///
/// See {flags} for pre-defined mount flags.
///
/// = See also
///
/// * flags
pub struct MountFlags(c_ulong);

impl BitOr for MountFlags {
    type Output = MountFlags;
    fn bitor(self, other: MountFlags) -> MountFlags {
        MountFlags(self.0 | other.0)
    }
}

impl BitAnd for MountFlags {
    type Output = MountFlags;
    fn bitand(self, other: MountFlags) -> MountFlags {
        MountFlags(self.0 & other.0)
    }
}

impl Not for MountFlags {
    type Output = MountFlags;
    fn not(self) -> MountFlags {
        MountFlags(!self.0)
    }
}

pub const MOUNT_NONE: MountFlags = MountFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: MountFlags = MountFlags($val);)*

        impl Debug for MountFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
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
    #[doc = "Mount the filesystem read-only.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_RDONLY therein"]
    flag MOUNT_READ_ONLY = MS_RDONLY;

    #[doc = "Don't respect set-user-id and set-group-id flags on the filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_NOSUID therein"]
    flag MOUNT_NO_SET_ID = MS_NOSUID;

    #[doc = "Don't allow access to devices on this filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_NODEV therein"]
    flag MOUNT_NO_DEVICE_ACCESS = MS_NODEV;

    #[doc = "Don't allow execution of programs on this filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_NOEXEC therein"]
    flag MOUNT_NO_EXEC = MS_NOEXEC;

    #[doc = "Flush all data and meta-data changes to this filesystem to the disk \
             immediately.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_SYNCHRONOUS therein"]
    flag MOUNT_SYNC = MS_SYNCHRONOUS;

    #[doc = "Perform a remount operation.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_REMOUNT therein"]
    flag MOUNT_REMOUNT = MS_REMOUNT;

    #[doc = "Allow mandatory locking on the monut point.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_MANBLOCK therein"]
    flag MOUNT_MANDATORY_LOCKING = MS_MANDLOCK;

    #[doc = "Make directory changes on this filesystem synchonous.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_DIRSYNC therein"]
    flag MOUNT_DIR_SYNC = MS_DIRSYNC;

    #[doc = "Don't update the access times of files on this filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_NOATIME therein"]
    flag MOUNT_NO_ACCESS_TIME = MS_NOATIME;

    #[doc = "Don't update the access times of directories on this filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_NODIRATIME therein"]
    flag MOUNT_NO_DIR_ACCESS_TIME = MS_NODIRATIME;

    #[doc = "Perform a bind operation.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_BIND therein"]
    flag MOUNT_BIND = MS_BIND;

    #[doc = "Atomically move a mount to another mount point.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_MOVE therein"]
    flag MOUNT_MOVE = MS_MOVE;

    #[doc = "Not documented."]
    flag MOUNT_REC = MS_REC;

    #[doc = "Omit certain warning messages from the kernel log.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_SILENT therein"]
    flag MOUNT_SILENT = MS_SILENT;

    #[doc = "Not documented."]
    flag MOUNT_POSIX_ACL = MS_POSIXACL;

    #[doc = "Not documented."]
    flag MOUNT_UNBINDABLE = MS_UNBINDABLE;

    #[doc = "Not documented."]
    flag MOUNT_PRIVATE = MS_PRIVATE;

    #[doc = "Not documented."]
    flag MOUNT_SLAVE = MS_SLAVE;

    #[doc = "Not documented."]
    flag MOUNT_SHARED = MS_SHARED;

    #[doc = "Perform an access time update after every access.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mount(2) and MS_STRICTATIME therein"]
    flag MOUNT_STRICT_ACCESS_TIME = MS_STRICTATIME;

    #[doc = "Maintain changes to access/modification/status-change times in memory and \
             only update the inodes under special circumstances.\n"]
    #[doc = "= Remarks"]
    #[doc = ":lazy: link:man:mount(2)"]
    #[doc = "See the {lazy}[manual page] and MS_LAZYTIME therein for the details.\n"]
    #[doc = "== Kernel versions"]
    #[doc = "The required kernel version is 4.0.\n"]
    #[doc = "= See also"]
    #[doc = "* {lazy} and MS_LAZYTIME therein"]
    flag MOUNT_LAZY_TIME = MS_LAZYTIME;
}

impl MountFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MountFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MountFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MountFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
