// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File system information and mounting.

pub use lrs_fs::{sync_all};
pub use lrs_fs::mount::{mount, MountFlags};
pub use lrs_fs::unmount::{unmount, UnmountFlags};
pub use lrs_fs::info::{FileSystemInfo};
pub use lrs_fs::info::mount::{Flags};
pub use lrs_fs::info::types::{FileSystem};

/// Mount and unmount flags.
///
/// = Description
///
/// This module contains pre-defined flags for mount and unmount operations.
pub mod flags {
    pub use lrs_fs::mount::{
        MOUNT_NONE, MOUNT_READ_ONLY, MOUNT_NO_SET_ID, MOUNT_NO_DEVICE_ACCESS,
        MOUNT_NO_EXEC, MOUNT_SYNC, MOUNT_REMOUNT, MOUNT_MANDATORY_LOCKING, MOUNT_DIR_SYNC,
        MOUNT_NO_ACCESS_TIME, MOUNT_NO_DIR_ACCESS_TIME, MOUNT_BIND, MOUNT_MOVE, MOUNT_REC,
        MOUNT_SILENT, MOUNT_POSIX_ACL, MOUNT_UNBINDABLE, MOUNT_PRIVATE, MOUNT_SLAVE,
        MOUNT_SHARED, MOUNT_STRICT_ACCESS_TIME, MOUNT_LAZY_TIME,
    };

    pub use lrs_fs::unmount::{
        UNMOUNT_NONE, UNMOUNT_FORCE, UNMOUNT_LAZY, UNMOUNT_EXPIRE, UNMOUNT_NO_FOLLOW,
    };
}

/// Filesystem types.
///
/// = Description
///
/// This module contains constants identifying filesystems.
pub mod types {
    pub use lrs_fs::info::types::{
        ADFS, AFFS, AFS, ANON_INODE_FS, AUFS, AUTOFS, BDEVFS, BEFS, BFS, BINFMTFS, BTRFS,
        CEPH, CGROUP, CIFS, CODA, COH, CONFIGFS, CRAMFS, CRAMFS_WEND, DEBUGFS, DEVFS,
        DEVPTS, ECRYPTFS, EFIVARFS, EFS, EXOFS, EXT, EXT2, EXT2_OLD, F2FS, FAT, FHGFS,
        FUSEBLK, FUSECTL, FUTEXFS, GFS, GPFS, HFS, HFS_PLUS, HFS_X, HOSTFS, HPFS,
        HUGETLBFS, INOTIFYFS, ISOFS, ISOFS_R_WIN, ISOFS_WIN, JFFS, JFFS2, JFS, KAFS,
        LOGFS, LUSTRE, MINIX, MINIX_30, MINIX_V2, MINIX_V2_30, MINIX_V3, MQUEUE, MSDOS,
        MTD_INODE_FS, NCP, NFS, NFSD, NILFS, NTFS, OCFS2, OPENPROM, PANFS, PIPEFS, PROC,
        PSTOREFS, QNX4, QNX6, RAMFS, REISERFS, ROMFS, RPC_PIPEFS, SECURITYFS, SELINUX,
        SMACK, SMB, SNFS, SOCKFS, SQUASHFS, SYSFS, SYSV2, SYSV4, TMPFS, UBIFS, UDF, UFS,
        UFS_BYTESWAPPED, USBDEVFS, V9FS, VMHGFS, VXFS, VZFS, XENFS, XENIX, XFS, XIAFS,
        ZFS,
    };
}
