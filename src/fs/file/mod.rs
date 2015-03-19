use std::os::unix::{OsStrExt};
use std::ffi::{AsOsStr};
use std::{mem};

use result::{Result};
use errno::{self};
use libc::{self, c_int, c_void, size_t, open64, read, write, close, pread64, off64_t,
           lseek64, F_SETFL, F_GETFL, fcntl, F_DUPFD_CLOEXEC, F_GETFD, F_SETFD, pwrite64,
           readv, iovec, c_uint, writev, preadv64, pwritev64, ftruncate64, fsync,
           fdatasync, syncfs, posix_fadvise64, fstatfs64};
use rust::{AsLinuxPath, UIntRange};
use util::{retry};
use fs::info::{FileSystemInfo};

pub use self::flags::{Flags, Mode};

mod flags;

/// An opened file in a file system.
#[derive(Debug, Eq, PartialEq)]
pub struct File {
    fd: c_int,
    /// File has ownership of the file descriptor.
    owned: bool,
}

impl File {
    /// Open the file at path `path` in read mode.
    ///
    /// This is equivalent to `File::open` with the default flags.
    pub fn open_read<P: AsLinuxPath>(path: P) -> Result<File> {
        File::open(path, Flags::new())
    }

    /// A file on which every operation fails.
    pub fn invalid() -> File {
        File { fd: -1, owned: false }
    }

    /// Open the file at path `path` with the specified flags.
    ///
    /// ### Return value
    ///
    /// Returns the opened file or an error.
    pub fn open<P: AsLinuxPath>(path: P, flags: Flags) -> Result<File> {
        let path = path.as_linux_path().as_os_str().to_cstring().unwrap();
        let fd = match retry(|| unsafe {
            open64(path.as_ptr(), *flags, *flags.mode())
        }) {
            Ok(fd) => fd,
            // Due to a bug in the kernel, open returns WrongDeviceType instead of
            // NoSuchDevice.
            Err(errno::WrongDeviceType) => return Err(errno::NoSuchDevice),
            Err(e) => return Err(e),
        };
        Ok(File {
            fd: fd,
            owned: true,
        })
    }

    /// Reads bytes from the current read position into the buffer.
    ///
    /// ### Return value
    ///
    /// Returns the number of bytes read or an error. Zero indicates end of file.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        retry(||  unsafe {
            read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t)
        }).map(|r| r as usize)
    }

    /// Writes bytes to the current to position from the buffer.
    ///
    /// ### Return value
    ///
    /// Returns the number of bytes written or an error.
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        retry(||  unsafe {
            write(self.fd, buf.as_ptr() as *const c_void, buf.len() as size_t)
        }).map(|r| r as usize)
    }

    /// Closes the file descriptor.
    pub fn close(&mut self) -> Result<()> {
        if self.owned {
            let ret = unsafe { close(self.fd) };
            self.fd = -1;
            match ret {
                -1 => Err(errno::get()),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    /// Performs requested seek operation.
    ///
    /// ### Return value
    ///
    /// Returns the new position in the file or an error.
    pub fn seek(&self, pos: Seek) -> Result<i64> {
        let ret = unsafe { lseek64(self.fd, pos.offset(), pos.whence()) };
        match ret {
            -1 => Err(errno::get()),
            _ => Ok(ret as i64),
        }
    }

    /// Creates a new file referring to the same file description.
    ///
    /// The `close on exec` flag will be set on the new file.
    ///
    /// ### Return value
    ///
    /// Returns the new file or an error.
    pub fn duplicate(&self) -> Result<File> {
        let new_fd = unsafe { fcntl(self.fd, F_DUPFD_CLOEXEC, 0 as c_int) };
        match new_fd {
            -1 => Err(errno::get()),
            _ => Ok(File { fd: new_fd, owned: true }),
        }
    }

    /// Retrieves the file description flags.
    ///
    /// The returned flags will contain the access mode flags and the file status flags.
    pub fn get_status_flags(&self) -> Result<Flags> {
        let ret = unsafe { fcntl(self.fd, F_GETFL) };
        match ret {
            -1 => Err(errno::get()),
            _ => Ok(Flags::from_int(ret)),
        }
    }

    /// Sets the file description flags.
    ///
    /// Only the file status flags can be modified.
    pub fn set_status_flags(&self, flags: Flags) -> Result<()> {
        let ret = unsafe { fcntl(self.fd, F_SETFL, *flags) };
        match ret {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }

    /// Returns whether the file has the `close on exec` flag set.
    pub fn is_close_on_exec(&self) -> Result<bool> {
        let ret = unsafe { fcntl(self.fd, F_GETFD) };
        match ret {
            -1 => Err(errno::get()),
            _ => Ok(ret & libc::O_CLOEXEC != 0)
        }
    }

    /// Modifies the `close on exec` flag of the file.
    pub fn set_close_on_exec(&self, val: bool) -> Result<()> {
        let mut ret = unsafe { fcntl(self.fd, F_GETFD) };
        if ret == -1 {
            return Err(errno::get());
        }
        ret = (ret & !libc::O_CLOEXEC) | (libc::O_CLOEXEC * val as c_int);
        ret = unsafe { fcntl(self.fd, F_SETFD, ret) };
        match ret {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }

    /// Reads bytes from the offset into the buffer.
    ///
    /// The return value and errors are the same as for `read` and `seek`.
    pub fn read_at(&self, buf: &mut [u8], off: i64) -> Result<usize> {
        retry(||  unsafe {
            pread64(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t,
                    off as off64_t)
        }).map(|r| r as usize)
    }

    /// Writes bytes to the offset from the buffer.
    ///
    /// The return value and errors are the same as for `write` and `seek`.
    pub fn write_at(&self, buf: &[u8], off: i64) -> Result<usize> {
        retry(||  unsafe {
            pwrite64(self.fd, buf.as_ptr() as *const c_void, buf.len() as size_t,
                     off as off64_t)
        }).map(|r| r as usize)
    }

    /// Reads bytes from the current read position into the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes read.
    pub fn scatter_read(&self, bufs: &mut [&mut [u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| unsafe {
            readv(self.fd, bufs.as_ptr() as *const iovec, bufs.len() as c_int)
        }).map(|r| r as usize)
    }

    /// Writes bytes to the current write position from the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes written.
    pub fn gather_write(&self, bufs: &[&[u8]]) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| unsafe {
            writev(self.fd, bufs.as_ptr() as *const iovec, bufs.len() as c_int)
        }).map(|r| r as usize)
    }

    /// Reads bytes from the offset into the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes read.
    pub fn scatter_read_at(&self, bufs: &mut [&mut [u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| unsafe {
            preadv64(self.fd, bufs.as_ptr() as *const iovec, bufs.len() as c_int,
                     off as off64_t)
        }).map(|r| r as usize)
    }

    /// Writes bytes to the offset from the buffers.
    ///
    /// ### Return value
    ///
    /// Returns the total number of bytes written.
    pub fn gather_write_at(&self, bufs: &[&[u8]], off: i64) -> Result<usize> {
        assert!(bufs.len() < (!0 as c_uint / 2) as usize);
        retry(|| unsafe {
            pwritev64(self.fd, bufs.as_ptr() as *const iovec, bufs.len() as c_int,
                      off as off64_t)
        }).map(|r| r as usize)
    }

    /// Changes the length of the file to the specified length.
    ///
    /// If the requested length is larger than the current length, a hole is created.
    pub fn set_len(&self, len: i64) -> Result<()> {
        retry(|| unsafe { ftruncate64(self.fd, len as off64_t) }).map(|_| ())
    }

    /// Flushes all data and metadata to the disk.
    pub fn sync(&self) -> Result<()> {
        match unsafe { fsync(self.fd) } {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }

    /// Flushes enough data to the disk that the content of the file can be read again.
    pub fn data_sync(&self) -> Result<()> {
        match unsafe { fdatasync(self.fd) } {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }

    /// Writes all data and metadata of the filesystem containing this file to the disk.
    pub fn sync_filesystem(&self) -> Result<()> {
        match unsafe { syncfs(self.fd) } {
            -1 => Err(errno::get()),
            _ => Ok(()),
        }
    }

    /// Advise the kernel that the specified range will have a certain usage pattern.
    pub fn advise<R: UIntRange<u64>>(&self, range: R, advice: Advice) -> Result<()> {
        let range = range.to_range();
        let len = match range.end {
            -1 => 0,
            _ => range.end - range.start,
        };
        match unsafe { posix_fadvise64(self.fd, range.start as off64_t, len as off64_t,
                                       advice.to_c_int()) } {
            0 => Ok(()),
            n => Err(errno::Errno(n)),
        }
    }

    /// Returns information about the file system of this file.
    pub fn fs_info(&self) -> Result<FileSystemInfo> {
        let mut buf = unsafe { mem::zeroed() };
        if retry(|| unsafe { fstatfs64(self.fd, &mut buf) }).is_ok() {
            Ok(FileSystemInfo(buf))
        } else {
            Err(errno::get())
        }
    }
}

impl ::std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        Ok(try!(File::read(self, buf)))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.owned {
            unsafe { close(self.fd); }
        }
    }
}

/// A seek operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Seek {
    /// Seek from the start of the file.
    Start(i64),
    /// Seek from the current position in the file.
    Cur(i64),
    /// Seek from the end of the file.
    End(i64),
    /// Seek to the first non-hole byte at or after the specified offset.
    Data(i64),
    /// Seek to the first hole at or after the specified offset.
    Hole(i64),
}

impl Seek {
    fn whence(self) -> c_int {
        match self {
            Seek::Start(..) => 0,
            Seek::Cur(..)   => 1,
            Seek::End(..)   => 2,
            Seek::Data(..)  => 3,
            Seek::Hole(..)  => 4,
        }
    }

    fn offset(self) -> off64_t {
        match self {
            Seek::Start(v) => v as off64_t,
            Seek::Cur(v)   => v as off64_t,
            Seek::End(v)   => v as off64_t,
            Seek::Data(v)  => v as off64_t,
            Seek::Hole(v)  => v as off64_t,
        }
    }
}

/// Advice used to optimize file access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Advice {
    /// Default.
    Normal,
    /// Optimize for random access.
    Random,
    /// Optimize for sequential access.
    Sequential,
    /// The range will be accessed soon.
    Need,
    /// The range will not be accessed.
    DontNeed,
    /// The range will be accessed only once.
    NoReuse,
}

impl Advice {
    fn to_c_int(self) -> c_int {
        match self {
            Advice::Normal => 0,
            Advice::Random => 1,
            Advice::Sequential => 2,
            Advice::Need => 3,
            Advice::DontNeed => 4,
            Advice::NoReuse => 5,
        }
    }
}
