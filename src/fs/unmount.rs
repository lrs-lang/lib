// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use core::ops::{BitOr, Not, BitAnd};
use cty::{c_int, PATH_MAX, MNT_FORCE, MNT_DETACH, MNT_EXPIRE, UMOUNT_NOFOLLOW};
use fmt::{Debug, Write};
use syscall::{self};
use str_three::{ToCString};
use alloc::{FbHeap};
use rmo::{Rmo};

/// Unmounts a filesystem.
///
/// [argument, dst]
/// The path of the mountpoint to be unmounted.
///
/// [argument, flags]
/// The flags to be used to unmount the filesystem.
///
/// = Remarks
///
/// :flags: link:lrs::fs::flags
///
/// See {flags} for pre-defined unmount flags. If the path is relative, it is interpreted
/// relative to the current working directory.
///
/// = Examples
///
/// The example in link:lrs::fs::mount[mount] shows how to bind-mount a directory `a` at
/// the path `b`. The following example unmounts `b`.
///
/// ----
/// unmount("b", UNMOUNT_LAZY).unwrap();
/// ----
///
/// = See also
///
/// * link:man:unmount(2)
/// * {flags}
pub fn unmount<P>(dst: P, flags: UnmountFlags) -> Result
    where P: ToCString,
{
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninit() };
    let dst: Rmo<_, FbHeap> = try!(dst.rmo_cstr(&mut buf));
    rv!(syscall::umount(&dst, flags.0))
}

/// Flags used for unmounting.
///
/// = Remarks
///
/// :flags: link:lrs::fs::flags
///
/// See {flags} for pre-defined unmount flags.
///
/// = See also
///
/// * flags
pub struct UnmountFlags(c_int);

impl BitOr for UnmountFlags {
    type Output = UnmountFlags;
    fn bitor(self, other: UnmountFlags) -> UnmountFlags {
        UnmountFlags(self.0 | other.0)
    }
}

impl BitAnd for UnmountFlags {
    type Output = UnmountFlags;
    fn bitand(self, other: UnmountFlags) -> UnmountFlags {
        UnmountFlags(self.0 & other.0)
    }
}

impl Not for UnmountFlags {
    type Output = UnmountFlags;
    fn not(self) -> UnmountFlags {
        UnmountFlags(!self.0)
    }
}

pub const UNMOUNT_NONE: UnmountFlags = UnmountFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: UnmountFlags = UnmountFlags($val);)*

        impl Debug for UnmountFlags {
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
    #[doc = "Unmount the filesystem even if the device is busy.\n"]
    #[doc = "= Remarks"]
    #[doc = "This can cause data loss and only works on NFS mounts.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:umount(2) and MNT_FORCE therein"]
    flag UNMOUNT_FORCE = MNT_FORCE;

    #[doc = "Disallow further access to the mount point and unmount the device when the \
             last pending access has been completed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:umount(2) and MNT_DETACH therein"]
    flag UNMOUNT_LAZY = MNT_DETACH;

    #[doc = "Set the \"expire\" flag on the filesystem.\n"]
    #[doc = "= Remarks"]
    #[doc = "The \"expire\" flag is automatically unset when the filesystem is accessed \
             again. If an unmount operation with this flag set is used and the \
             \"expire\" flag is already set on the filesystem, then the device is \
             unmounted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:umount(2) and MNT_EXPIRE therein"]
    flag UNMOUNT_EXPIRE = MNT_EXPIRE;

    #[doc = "Don't follow the unmount path if it's a symbolic link.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:umount(2) and UMOUNT_NOFOLLOW therein"]
    flag UNMOUNT_NO_FOLLOW = UMOUNT_NOFOLLOW;
}

impl UnmountFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: UnmountFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: UnmountFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: UnmountFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
