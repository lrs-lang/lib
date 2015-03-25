// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use imp::cty::{ino_t, nlink_t, mode_t, uid_t, gid_t, c_uint, dev_t, off_t, blksize_t,
               blkcnt_t, timespec, c_long};

// From musl/arch/x86_64/bits/stat.h
// See also linux/arch/x86/include/uapi/asm/stat.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct stat {
	pub st_dev: dev_t,
	pub st_ino: ino_t,
	pub st_nlink: nlink_t,

	pub st_mode:    mode_t,
	pub st_uid:     uid_t,
	pub st_gid:     gid_t,
	pub __pad0:     c_uint,
	pub st_rdev:    dev_t,
	pub st_size:    off_t,
	pub st_blksize: blksize_t,
	pub st_blocks:  blkcnt_t,

	pub st_atim: timespec,
	pub st_mtim: timespec,
	pub st_ctim: timespec,

    pub __unused: [c_long; 3],
}
