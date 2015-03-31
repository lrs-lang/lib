// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_ulong, fsblkcnt_t, fsfilcnt_t, fsid_t};

// From musl/arch/x86_64/bits/statfs.h
// See also linux/include/uapi/asm-generic/statfs.h

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct statfs {
	pub f_type:    c_ulong,
	pub f_bsize:   c_ulong,
	pub f_blocks:  fsblkcnt_t,
	pub f_bfree:   fsblkcnt_t,
	pub f_bavail:  fsblkcnt_t,
	pub f_files:   fsfilcnt_t,
	pub f_ffree:   fsfilcnt_t,
	pub f_fsid:    fsid_t,
	pub f_namelen: c_ulong,
	pub f_frsize:  c_ulong,
	pub f_flags:   c_ulong,
	pub f_spare:   [c_ulong; 4],
}
