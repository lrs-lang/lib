// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use core::cty::{self, c_int, mode_t};

/// Flags for opening and modifying a file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Flags {
    flags: c_int,
    mode: mode_t,
}

impl Flags {
    /// Create flags from an integer.
    ///
    /// If the `create` flag is set, the mode is set to `0`.
    pub fn from_int(flags: c_int) -> Flags {
        Flags {
            flags: flags,
            mode: 0,
        }
    }

    /// Return the mode used for creating a file.
    pub fn mode(&self) -> Mode {
        Mode { mode: self.mode }
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
        Flags::from_int(cty::O_RDONLY | cty::O_CLOEXEC)
    }

    /// Returns if the `readable` flags is set.
    pub fn is_readable(&self) -> bool {
        **self & cty::O_ACCMODE == cty::O_RDWR ||
            **self & cty::O_ACCMODE == cty::O_RDONLY
    }

    /// Returns if the `writable` flags is set.
    pub fn is_writable(&self) -> bool {
        **self & cty::O_ACCMODE == cty::O_RDWR ||
            **self & cty::O_ACCMODE == cty::O_RDONLY
    }

    /// Enables or disables the `readable` flag.
    pub fn set_readable(&mut self, val: bool) {
        **self = (**self & !cty::O_ACCMODE) | match (val, self.is_writable()) {
            (true,  true)  => cty::O_RDWR,
            (true,  false) => cty::O_RDONLY,
            (false, true)  => cty::O_WRONLY,
            (false, false) => 0,
        };
    }

    /// Enables or disables the `writable` flag.
    pub fn set_writable(&mut self, val: bool) {
        **self = (**self & !cty::O_ACCMODE) | match (val, self.is_readable()) {
            (true,  true)  => cty::O_RDWR,
            (true,  false) => cty::O_WRONLY,
            (false, true)  => cty::O_RDONLY,
            (false, false) => 0,
        };
    }

    /// Checks if the `close on exec` flag is set.
    ///
    /// If this flag is set, the associated file descriptor will be closed when `exec` is
    /// called.
    pub fn is_close_on_exec(&self) -> bool { **self & cty::O_CLOEXEC != 0 }

    /// Enables or disables the `close on exec` flag.
    pub fn set_close_on_exec(&mut self, val: bool) { self.set_bit(cty::O_CLOEXEC, val); }

    /// Checks if the `create` flag is set.
    ///
    /// If this flag is set and the file does not exist, it will be created with the
    /// specified mode.
    pub fn is_create(&self) -> bool { **self & cty::O_CREAT != 0 }

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

    /// Checks if the `bypass buffer` flag is set.
    ///
    /// If this flag is set, writes and reads bypass the kernel buffers and access the
    /// disk directly. Some limitations apply.
    pub fn is_bypass_buffer(&self) -> bool { **self & cty::O_DIRECT != 0 }

    /// Enables or disables the `bypass buffer` flag.
    pub fn set_bypass_buffer(&mut self, val: bool) { self.set_bit(cty::O_DIRECT, val); }

    /// Checks if the `only directory` flag is set.
    ///
    /// If this flag is set, opening fails if the path does not refer to a directory.
    pub fn is_only_directory(&self) -> bool { **self & cty::O_DIRECTORY != 0 }

    /// Enables or disables the `only directory` flag.
    pub fn set_only_directory(&mut self, val: bool) { self.set_bit(cty::O_DIRECTORY, val); }

    /// Checks if the `exclusive` flag is set.
    ///
    /// If this flag and the `create` flag is set and the file already exists, opening the
    /// file fails.
    pub fn is_exclusive(&self) -> bool { **self & cty::O_EXCL != 0 }

    /// Enables or disables the `exclusive` flag.
    pub fn set_exclusive(&mut self, val: bool) { self.set_bit(cty::O_EXCL, val); }

    /// Checks if the `access time update` flag is set.
    ///
    /// If this flag is not set, the access time won't be updated when the file is opened.
    pub fn is_access_time_update(&self) -> bool { **self & cty::O_NOATIME == 0 }

    /// Enables or disables the `access time update` flag.
    pub fn set_access_time_update(&mut self, val: bool) { self.set_bit(cty::O_NOATIME, !val); }

    /// Checks if the `controlling term` flag is set.
    ///
    /// If this flag is not set and a terminal is opened, the terminal will not become the
    /// controlling terminal of this process.
    pub fn is_controlling_term(&self) -> bool { **self & cty::O_NOCTTY == 0 }

    /// Enables or disables the `controlling term` flag.
    pub fn set_controlling_term(&mut self, val: bool) { self.set_bit(cty::O_NOCTTY, !val); }

    /// Checks if the `follow links` flag is set.
    ///
    /// If this flag is not set and the path refers to a symlink, the symlink will be
    /// opened.
    pub fn is_follow_links(&self) -> bool { **self & cty::O_NOFOLLOW == 0 }

    /// Enables or disables the `follow links` flag.
    pub fn set_follow_links(&mut self, val: bool) { self.set_bit(cty::O_NOFOLLOW, !val); }

    /// Checks if the `truncate` flag is set.
    ///
    /// If this flag and the `writable` flag is set and the file already exists, the file
    /// will be truncated to size `0`.
    pub fn is_truncate(&self) -> bool { **self & cty::O_TRUNC != 0 }

    /// Enables or disables the `truncate` flag.
    pub fn set_truncate(&mut self, val: bool) { self.set_bit(cty::O_TRUNC, val); }

    /// Checks if the `append` flag is set.
    ///
    /// If this flag is set, all writes will append to the end of the file.
    pub fn is_append(&self) -> bool { **self & cty::O_APPEND != 0 }

    /// Enables or disables the `append` flag.
    pub fn set_append(&mut self, val: bool) { self.set_bit(cty::O_APPEND, val); }

    /// Checks if the `signal io` flag is set.
    ///
    /// If this flag is set and the file becomes ready for reading or writing, a signal
    /// will be sent to the process.
    pub fn is_signal_io(&self) -> bool { **self & cty::O_ASYNC != 0 }

    /// Enables or disables the `signal io` flag.
    pub fn set_signal_io(&mut self, val: bool) { self.set_bit(cty::O_ASYNC, val); }

    /// Checks if the `data synchronized` flag is set.
    ///
    /// If this flag is set and writing succeeds, then all data necessary for reading the
    /// written data has been transferred to the disk hardware.
    pub fn is_data_synchronized(&self) -> bool { **self & cty::O_DSYNC != 0 }

    /// Enables or disables the `data synchronized` flag.
    pub fn set_data_synchronized(&mut self, val: bool) { self.set_bit(cty::O_DSYNC, val); }

    /// Checks if the `non blocking` flag is set.
    ///
    /// If this flag is set and reading or writing would block, an error will be returned
    /// instead.
    pub fn is_non_blocking(&self) -> bool { **self & cty::O_NONBLOCK != 0 }

    /// Enables or disables the `non blocking` flag.
    pub fn set_non_blocking(&mut self, val: bool) { self.set_bit(cty::O_NONBLOCK, val); }

    /// Checks if the `synchronized` flag is set.
    ///
    /// If this flag is set and writing succeeds, then all data has been transferred to
    /// the disk hardware.
    pub fn is_synchronized(&self) -> bool { **self & cty::O_SYNC != 0 }

    /// Enables or disables the `synchronized` flag.
    pub fn set_synchronized(&mut self, val: bool) { self.set_bit(cty::O_SYNC, val); }

    /// Checks if the `path fd` flag is set.
    ///
    /// If this flag is set, then the returned file will not refer to an opened file but
    /// merely indicate a location in the file system. Such a location can be used to
    /// perform a subset of the available operations.
    pub fn is_path_fd(&self) -> bool { **self & cty::O_PATH != 0 }

    /// Enables or disables the `path fd` flag.
    pub fn set_path_fd(&mut self, val: bool) { self.set_bit(cty::O_PATH, val); }

    /// Checks if the `temp file` flag is set.
    pub fn is_temp_file(&self) -> bool { **self & cty::O_TMPFILE != 0 }

    /// Enables or disables the `temp file` flag.
    ///
    /// If this flag is set, an unnamed file will be created in the directory specified by
    /// the path. The file will be automatically deleted when the file is closed.
    pub fn set_temp_file(&mut self, val: bool) { self.set_bit(cty::O_TMPFILE, val); }

    fn set_bit(&mut self, bit: c_int, val: bool) {
        **self = (**self & !bit) | (bit * val as c_int);
    }
}

impl Deref for Flags {
    type Target = c_int;

    fn deref(&self) -> &c_int {
        &self.flags
    }
}

impl DerefMut for Flags {
    fn deref_mut(&mut self) -> &mut c_int {
        &mut self.flags
    }
}

/// The permissions of a file.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mode {
    mode: mode_t,
}

impl Mode {
    /// Create the permissions from an integer.
    pub fn from_mode(mode: mode_t) -> Mode {
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
        **self & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set user id` flag.
    pub fn set_set_user_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISUID, val);
    }

    /// Checks if the `set group id` bit is set.
    pub fn is_set_group_id(&self) -> bool {
        **self & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set group id` flag.
    pub fn set_set_group_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISGID, val);
    }

    /// Checks if the `sticky` bit is set.
    pub fn is_sticky(&self) -> bool {
        **self & cty::S_ISVTX != 0
    }

    /// Sets or unsets the `sticky` flag.
    pub fn set_sticky(&mut self, val: bool) {
        self.set_bit(cty::S_ISVTX, val);
    }

    /// Checks if the `owner readable` bit is set.
    pub fn is_owner_readable(&self) -> bool {
        **self & cty::S_IRUSR != 0
    }

    /// Sets or unsets the `owner readable` flag.
    pub fn set_owner_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRUSR, val);
    }

    /// Checks if the `owner writable` bit is set.
    pub fn is_owner_writable(&self) -> bool {
        **self & cty::S_IWUSR != 0
    }

    /// Sets or unsets the `owner writable` flag.
    pub fn set_owner_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWUSR, val);
    }

    /// Checks if the `owner executable` bit is set.
    pub fn is_owner_executable(&self) -> bool {
        **self & cty::S_IXUSR != 0
    }

    /// Sets or unsets the `owner executable` flag.
    pub fn set_owner_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXUSR, val);
    }

    /// Checks if the `group readable` bit is set.
    pub fn is_group_readable(&self) -> bool {
        **self & cty::S_IRGRP != 0
    }

    /// Sets or unsets the `group readable` flag.
    pub fn set_group_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRGRP, val);
    }

    /// Checks if the `group writable` bit is set.
    pub fn is_group_writable(&mut self) -> bool {
        **self & cty::S_IWGRP != 0
    }

    /// Sets or unsets the `group writable` flag.
    pub fn set_group_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWGRP, val);
    }

    /// Checks if the `group executable` bit is set.
    pub fn is_group_executable(&self) -> bool {
        **self & cty::S_IXGRP != 0
    }

    /// Sets or unsets the `group executable` flag.
    pub fn set_group_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXGRP, val);
    }

    /// Checks if the `world readable` bit is set.
    pub fn is_world_readable(&self) -> bool {
        **self & cty::S_IROTH != 0
    }

    /// Sets or unsets the `world readable` flag.
    pub fn set_world_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IROTH, val);
    }

    /// Checks if the `world writable` bit is set.
    pub fn is_world_writable(&self) -> bool {
        **self & cty::S_IWOTH != 0
    }

    /// Sets or unsets the `world writable` flag.
    pub fn set_world_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWOTH, val);
    }

    /// Checks if the `world executable` bit is set.
    pub fn is_world_executable(&self) -> bool {
        **self & cty::S_IXOTH != 0
    }

    /// Sets or unsets the `world executable` flag.
    pub fn set_world_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXOTH, val);
    }

    fn set_bit(&mut self, bit: mode_t, val: bool) {
        **self = (**self & !bit) | (bit * val as mode_t);
    }
}

impl Deref for Mode {
    type Target = mode_t;

    fn deref(&self) -> &mode_t {
        &self.mode
    }
}

impl DerefMut for Mode {
    fn deref_mut(&mut self) -> &mut mode_t {
        &mut self.mode
    }
}

