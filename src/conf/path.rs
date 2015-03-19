use result::{Result};
use libc::{c_int, sysconf};
use errno::{self};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PathConf(pub c_int);

impl PathConf {
    pub fn get<P: AsLinuxPath>(self, path: P) -> Result<i64> {
        errno::set(0);
        let path = path.as_linux_path().as_os_str().to_cstring().unwrap();
        let res = unsafe { pathconf(path.as_ptr(), self.0) };
        if res == -1 {
            let err = errno::get();
            match err.0 {
                0 => Ok(res as i64),
                _ => Err(err),
            }
        } else {
            Ok(res as i64)
        }
    }
}

pub const LINK_MAX:           PathConf = PathConf(0);
pub const MAX_CANON:          PathConf = PathConf(1);
pub const MAX_INPUT:          PathConf = PathConf(2);
pub const NAME_MAX:           PathConf = PathConf(3);
pub const PATH_MAX:           PathConf = PathConf(4);
pub const PIPE_BUF:           PathConf = PathConf(5);
pub const CHOWN_RESTRICTED:   PathConf = PathConf(6);
pub const NO_TRUNC:           PathConf = PathConf(7);
pub const VDISABLE:           PathConf = PathConf(8);
pub const SYNC_IO:            PathConf = PathConf(9);
pub const ASYNC_IO:           PathConf = PathConf(10);
pub const PRIO_IO:            PathConf = PathConf(11);
pub const SOCK_MAXBUF:        PathConf = PathConf(12);
pub const FILESIZEBITS:       PathConf = PathConf(13);
pub const REC_INCR_XFER_SIZE: PathConf = PathConf(14);
pub const REC_MAX_XFER_SIZE:  PathConf = PathConf(15);
pub const REC_MIN_XFER_SIZE:  PathConf = PathConf(16);
pub const REC_XFER_ALIGN:     PathConf = PathConf(17);
pub const ALLOC_SIZE_MIN:     PathConf = PathConf(18);
pub const SYMLINK_MAX:        PathConf = PathConf(19);
// pub const 2_SYMLINKS: ::libc::c_uint = 20;
