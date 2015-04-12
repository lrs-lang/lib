use core::cty::{c_ulong, c_int, MS_RDONLY, MS_NOSUID, MS_NODEV, MS_NOEXEC, MS_SYNCHRONOUS,
                MS_REMOUNT, MS_MANDLOCK, MS_DIRSYNC, MS_NOATIME, MS_NODIRATIME,
                MS_BIND, MS_MOVE, MS_REC, MS_SILENT, MS_POSIXACL, MS_UNBINDABLE,
                MS_PRIVATE, MS_SLAVE, MS_SHARED, MS_STRICTATIME, PATH_MAX, MNT_FORCE,
                MNT_DETACH, MNT_EXPIRE, UMOUNT_NOFOLLOW,};
use core::syscall::{self};
use core::string::{AsLinuxStr};
use core::ext::{AsLinuxPath};
use core::result::{Result};

use std::{mem};

/// Flags used when mounting a filesystem.
pub struct MountFlags(c_ulong);

impl MountFlags {
    /// Creates new MountFlags with the default flags set.
    ///
    /// NB: This means the value is 0 in the underlying representation, but we've switched
    /// some flags around so here are the default flags:
    ///
    /// - `set_user_id`
    /// - `device_access`
    /// - `exec`
    /// - `access_time`
    /// - `dir_access_time`
    /// - `bindable`
    pub fn new() -> MountFlags {
        MountFlags(0)
    }

    /// If this flag is set, then the filesystem is read-only.
    pub fn is_read_only(&self) -> bool { self.0 & MS_RDONLY != 0 }

    /// If this flag is not set, then set-user-id files have no effect on the filesystem.
    pub fn is_set_user_id(&self) -> bool { self.0 & MS_NOSUID == 0 }

    /// If this flag is not set, then devices on this filesystem cannot be accessed.
    pub fn is_device_access(&self) -> bool { self.0 & MS_NODEV == 0 }

    /// If this flag is not set, then no programs on this filesystem can be executed.
    pub fn is_exec(&self) -> bool { self.0 & MS_NOEXEC == 0 }

    /// If this flag is set, then all writes to this filesystem are synchronous.
    pub fn is_sync(&self) -> bool { self.0 & MS_SYNCHRONOUS != 0 }

    /// If this flag is set, then a remount operation is performed.
    pub fn is_remount(&self) -> bool { self.0 & MS_REMOUNT != 0 }

    /// If this flag is set, then mandatory locking is supported on the filesystem.
    pub fn is_mandatory_locking(&self) -> bool { self.0 & MS_MANDLOCK != 0 }

    /// If this flag is set, then directory changes on this filesystem are synchonous.
    pub fn is_dir_sync(&self) -> bool { self.0 & MS_DIRSYNC != 0 }

    /// If this flag is not set, then the access times of files on this filesystem are not
    /// updated.
    pub fn is_access_time(&self) -> bool { self.0 & MS_NOATIME == 0 }

    /// If this flag is not set, then the access times of directories on this filesystem
    /// are not updated.
    pub fn is_dir_access_time(&self) -> bool { self.0 & MS_NODIRATIME == 0 }

    /// If this flag is set, then a bind operation is performed.
    pub fn is_bind(&self) -> bool { self.0 & MS_BIND != 0 }

    /// If this flag is set, then a mount is atomically moved to another mount point.
    pub fn is_move(&self) -> bool { self.0 & MS_MOVE != 0 }

    /// Not documented.
    pub fn is_rec(&self) -> bool { self.0 & MS_REC != 0 }

    /// If this flag is set, then certain warning messages are omited from the kernel log.
    pub fn is_silent(&self) -> bool { self.0 & MS_SILENT != 0 }

    /// Something about ACL. Not documented.
    pub fn is_posix_acl(&self) -> bool { self.0 & MS_POSIXACL != 0 }

    /// Not documented.
    pub fn is_bindable(&self) -> bool { self.0 & MS_UNBINDABLE == 0 }

    /// Not documented.
    pub fn is_private(&self) -> bool { self.0 & MS_PRIVATE != 0 }

    /// Not documented.
    pub fn is_slave(&self) -> bool { self.0 & MS_SLAVE != 0 }

    /// Not documented.
    pub fn is_shared(&self) -> bool { self.0 & MS_SHARED != 0 }

    /// If this flag is set, then every single access to a file causes an access time
    /// update.
    pub fn is_strict_access_time(&self) -> bool { self.0 & MS_STRICTATIME != 0 }

    pub fn set_read_only(&mut          self, val: bool) -> &mut MountFlags { self.set_bit(MS_RDONLY,      val);  self }
    pub fn set_set_user_id(&mut        self, val: bool) -> &mut MountFlags { self.set_bit(MS_NOSUID,      !val); self }
    pub fn set_device_access(&mut      self, val: bool) -> &mut MountFlags { self.set_bit(MS_NODEV,       !val); self }
    pub fn set_exec(&mut               self, val: bool) -> &mut MountFlags { self.set_bit(MS_NOEXEC,      !val); self }
    pub fn set_sync(&mut               self, val: bool) -> &mut MountFlags { self.set_bit(MS_SYNCHRONOUS, val);  self }
    pub fn set_remount(&mut            self, val: bool) -> &mut MountFlags { self.set_bit(MS_REMOUNT,     val);  self }
    pub fn set_mandatory_locking(&mut  self, val: bool) -> &mut MountFlags { self.set_bit(MS_MANDLOCK,    val);  self }
    pub fn set_dir_sync(&mut           self, val: bool) -> &mut MountFlags { self.set_bit(MS_DIRSYNC,     val);  self }
    pub fn set_access_time(&mut        self, val: bool) -> &mut MountFlags { self.set_bit(MS_NOATIME,     !val); self }
    pub fn set_dir_access_time(&mut    self, val: bool) -> &mut MountFlags { self.set_bit(MS_NODIRATIME,  !val); self }
    pub fn set_bind(&mut               self, val: bool) -> &mut MountFlags { self.set_bit(MS_BIND,        val);  self }
    pub fn set_move(&mut               self, val: bool) -> &mut MountFlags { self.set_bit(MS_MOVE,        val);  self }
    pub fn set_rec(&mut                self, val: bool) -> &mut MountFlags { self.set_bit(MS_REC,         val);  self }
    pub fn set_silent(&mut             self, val: bool) -> &mut MountFlags { self.set_bit(MS_SILENT,      val);  self }
    pub fn set_posix_acl(&mut          self, val: bool) -> &mut MountFlags { self.set_bit(MS_POSIXACL,    val);  self }
    pub fn set_bindable(&mut           self, val: bool) -> &mut MountFlags { self.set_bit(MS_UNBINDABLE,  !val); self }
    pub fn set_private(&mut            self, val: bool) -> &mut MountFlags { self.set_bit(MS_PRIVATE,     val);  self }
    pub fn set_slave(&mut              self, val: bool) -> &mut MountFlags { self.set_bit(MS_SLAVE,       val);  self }
    pub fn set_shared(&mut             self, val: bool) -> &mut MountFlags { self.set_bit(MS_SHARED,      val);  self }
    pub fn set_strict_access_time(&mut self, val: bool) -> &mut MountFlags { self.set_bit(MS_STRICTATIME, val);  self }

    fn set_bit(&mut self, bit: c_ulong, val: bool) {
        self.0 = (self.0 & !bit) | (bit * val as c_ulong);
    }
}

/// Mounts a file `src` of type `ty` at `dst` with the flags `flags`.
///
/// The contents of the `data` field depend on the filesystem type.
pub fn mount<P, Q, R, S>(src: P, dst: Q, ty: R, flags: MountFlags, data: S) -> Result
    where P: AsLinuxPath,
          Q: AsLinuxPath,
          R: AsLinuxStr,
          S: AsLinuxStr
{
    let mut buf1: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf2: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let mut buf3: [u8; 256] = unsafe { mem::uninitialized() };
    let mut buf4: [u8; 256] = unsafe { mem::uninitialized() };
    let src = try!(src.to_cstr(&mut buf1));
    let dst = try!(dst.to_cstr(&mut buf2));
    let ty = try!(ty.to_cstr(&mut buf3));
    let data = try!(data.to_cstr(&mut buf4));
    rv!(syscall::mount(&src, &dst, &ty, flags.0, &data))
}

/// Flags used for unmounting.
pub struct UnmountFlags(c_int);

impl UnmountFlags {
    /// Creates new UnmountFlags with the default flags set.
    ///
    /// NB: This means the value is 0 in the underlying representation, but we've switched
    /// some flags around so here are the default flags:
    ///
    /// - `follow`
    pub fn new() -> UnmountFlags {
        UnmountFlags(0)
    }

    /// If this flag is set, then the unmount is performed even if the device is busy.
    ///
    /// This can cause data loss and only works on NFS mounts.
    pub fn is_force(&self) -> bool { self.0 & MNT_FORCE != 0 }

    /// If this flag is set, then the mount point cannot be accessed anymore and the
    /// device is unmounted when the last access is done.
    pub fn is_lazy(&self) -> bool { self.0 & MNT_DETACH != 0 }

    /// If this flag is set, then an "expire" flag is set on the filesystem.
    ///
    /// The "expire" flag is automatically unset when the filesystem is accessed again. If
    /// an unmount operation with this flag set is used and the "expire" flag is already
    /// set on the filesystem, then the device is unmounted.
    pub fn is_expire(&self) -> bool { self.0 & MNT_EXPIRE != 0 }

    /// If this flag is not set and `dst` is a symbolic link, then we don't follow the
    /// link.
    pub fn is_follow(&self) -> bool { self.0 & UMOUNT_NOFOLLOW == 0 }

    pub fn set_force(&mut  self, val: bool) -> &mut UnmountFlags { self.set_bit(MNT_FORCE,       val);  self }
    pub fn set_lazy(&mut   self, val: bool) -> &mut UnmountFlags { self.set_bit(MNT_DETACH,      val);  self }
    pub fn set_expire(&mut self, val: bool) -> &mut UnmountFlags { self.set_bit(MNT_EXPIRE,      val);  self }
    pub fn set_follow(&mut self, val: bool) -> &mut UnmountFlags { self.set_bit(UMOUNT_NOFOLLOW, !val); self }

    fn set_bit(&mut self, bit: c_int, val: bool) {
        self.0 = (self.0 & !bit) | (bit * val as c_int);
    }
}

/// Unmounts the device mounted at `dst` with the flags `flags`.
pub fn unmount<P: AsLinuxPath>(dst: P, flags: UnmountFlags) -> Result {
    let mut buf: [u8; PATH_MAX] = unsafe { mem::uninitialized() };
    let dst = try!(dst.to_cstr(&mut buf));
    rv!(syscall::umount(&dst, flags.0))
}
