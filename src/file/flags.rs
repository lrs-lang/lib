// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fmt, str};

use core::cty::{self, c_int, umode_t, S_IROTH, S_IWOTH, S_IXOTH};

/// Flags for opening and modifying a file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Flags {
    flags: c_int,
    mode: umode_t,
}

impl Flags {
    /// Return the mode used for creating a file.
    pub fn mode(&self) -> Option<Mode> {
        if self.is_create() {
            Some(Mode { mode: self.mode })
        } else {
            None
        }
    }

    /// Create new flags with the default flags set.
    ///
    /// The default flags are:
    ///
    /// - `readable`
    /// - `close on exec`
    /// - `access time update`
    /// - `controlling term`
    /// - `follow links`
    pub fn new() -> Flags {
        let mut flags = Flags { flags: 0, mode: 0 };
        flags.set_readable(true).set_close_on_exec(true);
        flags
    }

    /// If this flag is set and the file does not exist, it will be created with the
    /// specified mode.
    pub fn is_create(&self) -> bool { self.flags & cty::O_CREAT != 0 }

    /// Sets the `create` flag.
    ///
    /// The mode will be used to create the file.
    pub fn enable_create(&mut self, mode: Mode) {
        self.set_bit(cty::O_CREAT, true);
        self.mode = mode.mode;
    }

    /// Unsets the `create` flag.
    pub fn disable_create(&mut self) {
        self.set_bit(cty::O_CREAT, false);
    }

    pub fn is_readable(&self) -> bool {
        self.flags & cty::O_ACCMODE == cty::O_RDWR ||
            self.flags & cty::O_ACCMODE == cty::O_RDONLY
    }

    pub fn is_writable(&self) -> bool {
        self.flags & cty::O_ACCMODE == cty::O_RDWR ||
            self.flags & cty::O_ACCMODE == cty::O_WRONLY
    }

    /// If this flag is set, the associated file descriptor will be closed when `exec` is
    /// called.
    pub fn is_close_on_exec(&self) -> bool { self.flags & cty::O_CLOEXEC != 0 }

    /// If this flag is set, writes and reads bypass the kernel buffers and access the
    /// disk directly. Some limitations apply.
    pub fn is_bypass_buffer(&self) -> bool { self.flags & cty::O_DIRECT != 0 }

    /// If this flag is set, opening fails if the path does not refer to a directory.
    pub fn is_only_directory(&self) -> bool { self.flags & cty::O_DIRECTORY != 0 }

    /// If this flag and the `create` flag is set and the file already exists, opening the
    /// file fails.
    pub fn is_exclusive(&self) -> bool { self.flags & cty::O_EXCL != 0 }

    /// If this flag is not set, the access time won't be updated when the file is opened.
    pub fn is_access_time_update(&self) -> bool { self.flags & cty::O_NOATIME == 0 }

    /// If this flag is not set and a terminal is opened, the terminal will not become the
    /// controlling terminal of this process.
    pub fn is_controlling_term(&self) -> bool { self.flags & cty::O_NOCTTY == 0 }

    /// If this flag is not set and the path refers to a symlink, the symlink will be
    /// opened.
    pub fn is_follow_links(&self) -> bool { self.flags & cty::O_NOFOLLOW == 0 }

    /// If this flag and the `writable` flag is set and the file already exists, the file
    /// will be truncated to size `0`.
    pub fn is_truncate(&self) -> bool { self.flags & cty::O_TRUNC != 0 }

    /// If this flag is set, all writes will append to the end of the file.
    pub fn is_append(&self) -> bool { self.flags & cty::O_APPEND != 0 }

    /// If this flag is set and the file becomes ready for reading or writing, a signal
    /// will be sent to the process.
    pub fn is_signal_io(&self) -> bool { self.flags & cty::O_ASYNC != 0 }

    /// If this flag is set and writing succeeds, then all data necessary for reading the
    /// written data has been transferred to the disk hardware.
    pub fn is_data_synchronized(&self) -> bool { self.flags & cty::O_DSYNC != 0 }

    /// If this flag is set and reading or writing would block, an error will be returned
    /// instead.
    pub fn is_non_blocking(&self) -> bool { self.flags & cty::O_NONBLOCK != 0 }

    /// If this flag is set and writing succeeds, then all data has been transferred to
    /// the disk hardware.
    pub fn is_synchronized(&self) -> bool { self.flags & cty::O_SYNC != 0 }

    /// If this flag is set, then the returned file will not refer to an opened file but
    /// merely indicate a location in the file system. Such a location can be used to
    /// perform a subset of the available operations.
    pub fn is_path_fd(&self) -> bool { self.flags & cty::O_PATH != 0 }

    /// If this flag is set, an unnamed file will be created in the directory specified by
    /// the path. The file will be automatically deleted when the file is closed.
    pub fn is_temp_file(&self) -> bool { self.flags & cty::O_TMPFILE != 0 }

    pub fn set_readable(&mut self, val: bool) -> &mut Flags {
        self.flags = (self.flags & !cty::O_ACCMODE) | match (val, self.is_writable()) {
            (true,  true)  => cty::O_RDWR,
            (true,  false) => cty::O_RDONLY,
            (false, true)  => cty::O_WRONLY,
            (false, false) => 0,
        };
        self
    }

    pub fn set_writable(&mut self, val: bool) -> &mut Flags {
        self.flags = (self.flags & !cty::O_ACCMODE) | match (val, self.is_readable()) {
            (true,  true)  => cty::O_RDWR,
            (true,  false) => cty::O_WRONLY,
            (false, true)  => cty::O_RDONLY,
            (false, false) => 0,
        };
        self
    }

    pub fn set_close_on_exec(      &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_CLOEXEC,   val);  self }
    pub fn set_bypass_buffer(      &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_DIRECT,    val);  self }
    pub fn set_only_directory(     &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_DIRECTORY, val);  self }
    pub fn set_exclusive(          &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_EXCL,      val);  self }
    pub fn set_access_time_update( &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_NOATIME,   !val); self }
    pub fn set_controlling_term(   &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_NOCTTY,    !val); self }
    pub fn set_follow_links(       &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_NOFOLLOW,  !val); self }
    pub fn set_truncate(           &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_TRUNC,     val);  self }
    pub fn set_append(             &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_APPEND,    val);  self }
    pub fn set_signal_io(          &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_ASYNC,     val);  self }
    pub fn set_data_synchronized(  &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_DSYNC,     val);  self }
    pub fn set_non_blocking(       &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_NONBLOCK,  val);  self }
    pub fn set_synchronized(       &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_SYNC,      val);  self }
    pub fn set_path_fd(            &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_PATH,      val);  self }
    pub fn set_temp_file(          &mut self, val: bool) -> &mut Flags { self.set_bit(cty::O_TMPFILE,   val);  self }

    fn set_bit(&mut self, bit: c_int, val: bool) {
        self.flags = (self.flags & !bit) | (bit * val as c_int);
    }
}

pub fn flags_from_int(f: c_int) -> Flags {
    Flags { flags: f, mode: 0 }
}

pub fn flags_to_int(f: Flags) -> c_int {
    f.flags
}

/// The permissions of a file.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode {
    mode: umode_t,
}

impl Mode {
    /// Create the permissions from an integer.
    pub fn from_mode(mode: umode_t) -> Mode {
        Mode { mode: mode }
    }

    /// Create permissions will all bits unset.
    pub fn empty() -> Mode {
        Mode::from_mode(0)
    }

    /// Create permissions with the default bits for a file:
    ///
    /// - `owner readable`
    /// - `owner writable`
    /// - `group readable`
    /// - `world readable`
    pub fn new_file() -> Mode {
        Mode::from_mode(0o644)
    }

    /// Create permissions with the default bits for a directory:
    ///
    /// - `owner readable`
    /// - `owner writable`
    /// - `owner executable`
    /// - `group readable`
    /// - `group executable`
    /// - `world readable`
    /// - `world executable`
    pub fn new_directory() -> Mode {
        Mode::from_mode(0o755)
    }

    /// Checks if the `set user id` bit is set.
    pub fn is_set_user_id(&self) -> bool {
        self.mode & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set user id` flag.
    pub fn set_set_user_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISUID, val);
    }

    /// Checks if the `set group id` bit is set.
    pub fn is_set_group_id(&self) -> bool {
        self.mode & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set group id` flag.
    pub fn set_set_group_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISGID, val);
    }

    /// Checks if the `sticky` bit is set.
    pub fn is_sticky(&self) -> bool {
        self.mode & cty::S_ISVTX != 0
    }

    /// Sets or unsets the `sticky` flag.
    pub fn set_sticky(&mut self, val: bool) {
        self.set_bit(cty::S_ISVTX, val);
    }

    /// Checks if the `owner readable` bit is set.
    pub fn is_owner_readable(&self) -> bool {
        self.mode & cty::S_IRUSR != 0
    }

    /// Sets or unsets the `owner readable` flag.
    pub fn set_owner_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRUSR, val);
    }

    /// Checks if the `owner writable` bit is set.
    pub fn is_owner_writable(&self) -> bool {
        self.mode & cty::S_IWUSR != 0
    }

    /// Sets or unsets the `owner writable` flag.
    pub fn set_owner_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWUSR, val);
    }

    /// Checks if the `owner executable` bit is set.
    pub fn is_owner_executable(&self) -> bool {
        self.mode & cty::S_IXUSR != 0
    }

    /// Sets or unsets the `owner executable` flag.
    pub fn set_owner_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXUSR, val);
    }

    /// Checks if the `group readable` bit is set.
    pub fn is_group_readable(&self) -> bool {
        self.mode & cty::S_IRGRP != 0
    }

    /// Sets or unsets the `group readable` flag.
    pub fn set_group_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRGRP, val);
    }

    /// Checks if the `group writable` bit is set.
    pub fn is_group_writable(&self) -> bool {
        self.mode & cty::S_IWGRP != 0
    }

    /// Sets or unsets the `group writable` flag.
    pub fn set_group_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWGRP, val);
    }

    /// Checks if the `group executable` bit is set.
    pub fn is_group_executable(&self) -> bool {
        self.mode & cty::S_IXGRP != 0
    }

    /// Sets or unsets the `group executable` flag.
    pub fn set_group_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXGRP, val);
    }

    /// Checks if the `world readable` bit is set.
    pub fn is_world_readable(&self) -> bool {
        self.mode & cty::S_IROTH != 0
    }

    /// Sets or unsets the `world readable` flag.
    pub fn set_world_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IROTH, val);
    }

    /// Checks if the `world writable` bit is set.
    pub fn is_world_writable(&self) -> bool {
        self.mode & cty::S_IWOTH != 0
    }

    /// Sets or unsets the `world writable` flag.
    pub fn set_world_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWOTH, val);
    }

    /// Checks if the `world executable` bit is set.
    pub fn is_world_executable(&self) -> bool {
        self.mode & cty::S_IXOTH != 0
    }

    /// Sets or unsets the `world executable` flag.
    pub fn set_world_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXOTH, val);
    }

    fn set_bit(&mut self, bit: umode_t, val: bool) {
        self.mode = (self.mode & !bit) | (bit * val as umode_t);
    }
}

pub fn mode_to_int(m: Mode) -> umode_t {
    m.mode
}

impl fmt::Debug for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! w {
            ($cond:expr, $c:expr) => { try!(if $cond { fmt.write_str($c) } else { fmt.write_str("-") }) }
        };
        w!(self.is_owner_readable(), "r");
        w!(self.is_owner_writable(), "w");
        match (self.is_owner_executable(), self.is_set_user_id()) {
            (true,  true)  => try!(fmt.write_str("s")),
            (true,  false) => try!(fmt.write_str("x")),
            (false, _)     => try!(fmt.write_str("-")),
        }
        w!(self.is_group_readable(), "r");
        w!(self.is_group_writable(), "w");
        match (self.is_group_executable(), self.is_set_group_id()) {
            (true,  true)  => try!(fmt.write_str("s")),
            (true,  false) => try!(fmt.write_str("x")),
            (false, _)     => try!(fmt.write_str("-")),
        }
        w!(self.is_world_readable(), "r");
        w!(self.is_world_writable(), "w");
        match (self.is_world_executable(), self.is_sticky()) {
            (true,  true)  => try!(fmt.write_str("t")),
            (true,  false) => try!(fmt.write_str("x")),
            (false, true)  => try!(fmt.write_str("T")),
            (false, false) => try!(fmt.write_str("-")),
        }
        Ok(())
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Mode as fmt::Debug>::fmt(self, fmt)
    }
}

impl str::FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Mode, ()> {
        if s.len() != 9 {
            return Err(());
        }
        let s = s.as_bytes();
        let mut mode = Mode::empty();
        match s[0] {
            b'r' => mode.set_owner_readable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[1] {
            b'w' => mode.set_owner_writable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[2] {
            b's' => {
                mode.set_owner_executable(true);
                mode.set_set_user_id(true);
            },
            b'x' => mode.set_owner_executable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[3] {
            b'r' => mode.set_group_readable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[4] {
            b'w' => mode.set_group_writable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[5] {
            b's' => {
                mode.set_group_executable(true);
                mode.set_set_group_id(true);
            },
            b'x' => mode.set_group_executable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[6] {
            b'r' => mode.set_world_readable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[7] {
            b'w' => mode.set_world_writable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[8] {
            b't' => {
                mode.set_world_executable(true);
                mode.set_sticky(true);
            },
            b'x' => mode.set_world_executable(true),
            b'T' => mode.set_sticky(true),
            b'-' => { },
            _ => return Err(()),
        }
        Ok(mode)
    }
}

/// A way to access a file.
///
/// This type is used to check whether a file can be accessed with a certain set of
/// permissions.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccessMode {
    mode: umode_t,
}

impl AccessMode {
    /// Create an access mode with all bits unset.
    pub fn empty() -> AccessMode {
        AccessMode { mode: 0 }
    }

    /// Checks if the `readable` bit is set.
    pub fn is_readable(&self) -> bool {
        self.mode & S_IROTH != 0
    }

    /// Sets or unsets the `readable` flag.
    pub fn set_readable(&mut self, val: bool) {
        self.set_bit(S_IROTH, val);
    }

    /// Checks if the `writable` bit is set.
    pub fn is_writable(&self) -> bool {
        self.mode & S_IWOTH != 0
    }

    /// Sets or unsets the `writable` flag.
    pub fn set_writable(&mut self, val: bool) {
        self.set_bit(S_IWOTH, val);
    }

    /// Checks if the `executable` bit is set.
    pub fn is_executable(&self) -> bool {
        self.mode & S_IXOTH != 0
    }

    /// Sets or unsets the `executable` flag.
    pub fn set_executable(&mut self, val: bool) {
        self.set_bit(S_IXOTH, val);
    }

    fn set_bit(&mut self, bit: umode_t, val: bool) {
        self.mode = (self.mode & !bit) | (bit * val as umode_t);
    }
}

impl fmt::Debug for AccessMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! w {
            ($cond:expr, $c:expr) => { try!(if $cond { fmt.write_str($c) } else { fmt.write_str("-") }) }
        };
        w!(self.is_readable(), "r");
        w!(self.is_writable(), "w");
        w!(self.is_executable(), "x");
        Ok(())
    }
}

impl fmt::Display for AccessMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <AccessMode as fmt::Debug>::fmt(self, fmt)
    }
}

impl str::FromStr for AccessMode {
    type Err = ();

    fn from_str(s: &str) -> Result<AccessMode, ()> {
        if s.len() != 3 {
            return Err(());
        }
        let s = s.as_bytes();
        let mut mode = AccessMode::empty();
        match s[0] {
            b'r' => mode.set_readable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[1] {
            b'w' => mode.set_writable(true),
            b'-' => { },
            _ => return Err(()),
        }
        match s[2] {
            b'x' => mode.set_executable(true),
            b'-' => { },
            _ => return Err(()),
        }
        Ok(mode)
    }
}

pub fn access_mode_to_int(mode: AccessMode) -> umode_t {
    mode.mode
}
