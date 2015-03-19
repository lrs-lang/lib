use super::{c_ulong, fsblkcnt_t, fsfilcnt_t};
use imp::cty::{fsid_t};

// From musl/arch/x86_64/bits/statfs.h

#[repr(u8)]
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
