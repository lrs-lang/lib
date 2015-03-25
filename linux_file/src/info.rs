use std::{fmt};

use core::cty::{stat};

//pub struct stat {
//	pub st_dev: dev_t,
//	pub st_ino: ino_t,
//	pub st_nlink: nlink_t,
//
//	pub st_mode:    mode_t,
//	pub st_uid:     uid_t,
//	pub st_gid:     gid_t,
//	pub __pad0:     c_uint,
//	pub st_rdev:    dev_t,
//	pub st_size:    off_t,
//	pub st_blksize: blksize_t,
//	pub st_blocks:  blkcnt_t,
//
//	pub st_atim: timespec,
//	pub st_mtim: timespec,
//	pub st_ctim: timespec,
//
//    pub __unused: [c_long; 3],
//}

pub struct Info(stat);

