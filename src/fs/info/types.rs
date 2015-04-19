// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::fmt::{Debug};
use base::io::{Write};
use base::cty::{c_ulong};

/// A filesystem type.
#[derive(Copy, Eq)]
pub struct FileSystem(pub c_ulong);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(pub const $name: FileSystem = FileSystem($val);)*
        impl Debug for FileSystem {
            fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
                match *self {
                    $($name => w.write($str).ignore_ok(),)*
                    x => write!(w, "Unknown(0x{:X})", x.0),
                }
            }
        }
    }
}

create! {
    ADFS            = (0xADF5,     b"ADFS"),
    AFFS            = (0xADFF,     b"AFFS"),
    AFS             = (0x5346414F, b"AFS"),
    ANON_INODE_FS   = (0x09041934, b"ANON_INODE_FS"),
    AUFS            = (0x61756673, b"AUFS"),
    AUTOFS          = (0x0187,     b"AUTOFS"),
    BEFS            = (0x42465331, b"BEFS"),
    BDEVFS          = (0x62646576, b"BDEVFS"),
    BFS             = (0x1BADFACE, b"BFS"),
    BINFMTFS        = (0x42494E4D, b"BINFMTFS"),
    BTRFS           = (0x9123683E, b"Btrfs"),
    CEPH            = (0x00C36400, b"CEPH"),
    CGROUP          = (0x0027E0EB, b"CGROUP"),
    CIFS            = (0xFF534D42, b"CIFS"),
    CODA            = (0x73757245, b"CODA"),
    COH             = (0x012FF7B7, b"COH"),
    CONFIGFS        = (0x62656570, b"CONFIGFS"),
    CRAMFS          = (0x28CD3D45, b"CRAMFS"),
    CRAMFS_WEND     = (0x453DCD28, b"CRAMFS_WEND"),
    DEBUGFS         = (0x64626720, b"DEBUGFS"),
    DEVFS           = (0x1373,     b"devfs"),
    DEVPTS          = (0x1CD1,     b"DEVPTS"),
    ECRYPTFS        = (0xF15F,     b"ECRYPTFS"),
    EFIVARFS        = (0xDE5E81E4, b"EFIVARFS"),
    EFS             = (0x00414A53, b"EFS"),
    EXOFS           = (0x5DF5,     b"EXOFS"),
    EXT             = (0x137D,     b"EXT"),
    EXT2            = (0xEF53,     b"ext2"),
    EXT2_OLD        = (0xEF51,     b"EXT2_OLD"),
    F2FS            = (0xF2F52010, b"F2FS"),
    FAT             = (0x4006,     b"FAT"),
    FHGFS           = (0x19830326, b"FHGFS"),
    FUSEBLK         = (0x65735546, b"FUSEBLK"),
    FUSECTL         = (0x65735543, b"FUSECTL"),
    FUTEXFS         = (0x0BAD1DEA, b"FUTEXFS"),
    GFS             = (0x01161970, b"GFS"),
    GPFS            = (0x47504653, b"GPFS"),
    HFS             = (0x4244,     b"HFS"),
    HFS_PLUS        = (0x482B,     b"HFS_PLUS"),
    HFS_X           = (0x4858,     b"HFS_X"),
    HOSTFS          = (0x00C0FFEE, b"HOSTFS"),
    HPFS            = (0xF995E849, b"HPFS"),
    HUGETLBFS       = (0x958458F6, b"HUGETLBFS"),
    MTD_INODE_FS    = (0x11307854, b"MTD_INODE_FS"),
    INOTIFYFS       = (0x2BAD1DEA, b"INOTIFYFS"),
    ISOFS           = (0x9660,     b"ISOFS"),
    ISOFS_R_WIN     = (0x4004,     b"ISOFS_R_WIN"),
    ISOFS_WIN       = (0x4000,     b"ISOFS_WIN"),
    JFFS            = (0x07C0,     b"JFFS"),
    JFFS2           = (0x72B6,     b"JFFS2"),
    JFS             = (0x3153464A, b"JFS"),
    KAFS            = (0x6B414653, b"KAFS"),
    LOGFS           = (0xC97E8168, b"LOGFS"),
    LUSTRE          = (0x0BD00BD0, b"LUSTRE"),
    MINIX           = (0x137F,     b"MINIX"),
    MINIX_30        = (0x138F,     b"MINIX_30"),
    MINIX_V2        = (0x2468,     b"MINIX_V2"),
    MINIX_V2_30     = (0x2478,     b"MINIX_V2_30"),
    MINIX_V3        = (0x4D5A,     b"MINIX_V3"),
    MQUEUE          = (0x19800202, b"MQUEUE"),
    MSDOS           = (0x4D44,     b"MSDOS"),
    NCP             = (0x564C,     b"NCP"),
    NFS             = (0x6969,     b"NFS"),
    NFSD            = (0x6E667364, b"NFSD"),
    NILFS           = (0x3434,     b"NILFS"),
    NTFS            = (0x5346544E, b"NTFS"),
    OPENPROM        = (0x9FA1,     b"OPENPROM"),
    OCFS2           = (0x7461636F, b"OCFS2"),
    PANFS           = (0xAAD7AAEA, b"PANFS"),
    PIPEFS          = (0x50495045, b"PIPEFS"),
    PROC            = (0x9FA0,     b"proc"),
    PSTOREFS        = (0x6165676C, b"PSTOREFS"),
    QNX4            = (0x002F,     b"QNX4"),
    QNX6            = (0x68191122, b"QNX6"),
    RAMFS           = (0x858458F6, b"RAMFS"),
    REISERFS        = (0x52654973, b"ReiserFS"),
    ROMFS           = (0x7275,     b"ROMFS"),
    RPC_PIPEFS      = (0x67596969, b"RPC_PIPEFS"),
    SECURITYFS      = (0x73636673, b"SECURITYFS"),
    SELINUX         = (0xF97CFF8C, b"SELINUX"),
    SMACK           = (0x43415D53, b"SMACK"),
    SMB             = (0x517B,     b"SMB"),
    SNFS            = (0xBEEFDEAD, b"SNFS"),
    SOCKFS          = (0x534F434B, b"SOCKFS"),
    SQUASHFS        = (0x73717368, b"SQUASHFS"),
    SYSFS           = (0x62656572, b"sysfs"),
    SYSV2           = (0x012FF7B6, b"SYSV2"),
    SYSV4           = (0x012FF7B5, b"SYSV4"),
    TMPFS           = (0x01021994, b"tmpfs"),
    UBIFS           = (0x24051905, b"UBIFS"),
    UDF             = (0x15013346, b"UDF"),
    UFS             = (0x00011954, b"UFS"),
    UFS_BYTESWAPPED = (0x54190100, b"UFS_BYTESWAPPED"),
    USBDEVFS        = (0x9FA2,     b"USBDEVFS"),
    V9FS            = (0x01021997, b"V9FS"),
    VMHGFS          = (0xBACBACBC, b"VMHGFS"),
    VXFS            = (0xA501FCF5, b"VXFS"),
    VZFS            = (0x565A4653, b"VZFS"),
    XENFS           = (0xABBA1974, b"XENFS"),
    XENIX           = (0x012FF7B4, b"XENIX"),
    XFS             = (0x58465342, b"XFS"),
    XIAFS           = (0x012FD16D, b"XIAFS"),
    ZFS             = (0x2FC12FC1, b"ZFS"),
}
