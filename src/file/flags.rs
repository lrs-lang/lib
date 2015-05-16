// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Display, Write};
use base::{error};
use parse::{Parsable};
use cty::{
    self, c_int, umode_t, S_IROTH, S_IWOTH, S_IXOTH, O_CLOEXEC, O_DIRECT, O_DIRECTORY,
    O_EXCL, O_NOATIME, O_NOCTTY, O_NOFOLLOW, O_TRUNC, O_APPEND, O_ASYNC, O_DSYNC,
    O_NONBLOCK, O_SYNC, O_PATH, O_TMPFILE, O_RDWR, O_RDONLY, O_WRONLY, O_LARGEFILE,
    O_CREAT,
};

/// Flags for opening and modifying a file.
#[derive(Pod, Eq)]
pub struct FileFlags(pub c_int);

impl BitOr for FileFlags {
    type Output = FileFlags;
    fn bitor(self, other: FileFlags) -> FileFlags {
        FileFlags(self.0 | other.0)
    }
}

impl BitAnd for FileFlags {
    type Output = FileFlags;
    fn bitand(self, other: FileFlags) -> FileFlags {
        FileFlags(self.0 & other.0)
    }
}

impl Not for FileFlags {
    type Output = FileFlags;
    fn not(self) -> FileFlags {
        FileFlags(!self.0)
    }
}

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: FileFlags = FileFlags($val);)*

        impl Debug for FileFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & $val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                let _ = first;
                Ok(())
            }
        }
    }
}

create! {
    #[doc = "Create a regulary file if it doesn't already exist.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_CREAT therein"]
    flag FILE_CREATE = O_CREAT;

    #[doc = "Open the file in read-only mode.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag cannot be combined with FILE_WRITE_ONLY to open a file for \
             reading and writing. Use FILE_READ_WRITE instead.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_RDONLY therein"]
    flag FILE_READ_ONLY = O_RDONLY;

    #[doc = "Open the file in write-only mode.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag cannot be combined with FILE_READ_ONLY to open a file for \
             reading and writing. Use FILE_READ_WRITE instead.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_WRONLY therein"]
    flag FILE_WRITE_ONLY = O_WRONLY;

    #[doc = "Open the file for reading and writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_RDWR therein"]
    flag FILE_READ_WRITE = O_RDWR;

    #[doc = "Close the file when `exec` is called.\n"]
    #[doc = "= Remarks"]
    #[doc = ":setcloexec: link:lrs::file::File::set_close_on_exec[set_close_on_exec]"]
    #[doc = "It's not possible to *not* use this flag when opening a file as lrs will \
             always add this flag before performing an open syscall. If you want to \
             prevent a file from being closed after an `exec` call use {setcloexec}.\n"]
    #[doc = "The rationale is that setting the close-on-exec flag is a racy operation \
             while unsetting it is not.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_CLOEXEC therein"]
    #[doc = "* {setcloexec}"]
    flag FILE_CLOSE_ON_EXEC = O_CLOEXEC;

    #[doc = "Bypass kernel buffers and write directly to the disk.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DIRECT therein"]
    flag FILE_BYPASS_BUFFER = O_DIRECT;

    #[doc = "Fail opening the file if it's not a directory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DIRECTORY therein"]
    flag FILE_ONLY_DIRECTORY = O_DIRECTORY;

    #[doc = "Fail creating a new file if it already exists.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_EXCL therein"]
    flag FILE_EXCLUSIVE = O_EXCL;

    #[doc = "Don't update the access time of the file.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NOATIME therein"]
    flag FILE_NO_ACCESS_TIME_UPDATE = O_NOATIME;

    #[doc = "Don't make the opened file the controlling terminal of this process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NOCTTY therein"]
    flag FILE_NO_CONTROLLING_TERM = O_NOCTTY;

    #[doc = "Don't follow symlinks during the opening process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NOFOLLOW therein"]
    flag FILE_DONT_FOLLOW_LINKS = O_NOFOLLOW;

    #[doc = "Truncate the file to size `0` for writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_TRUNC therein"]
    flag FILE_TRUNCATE = O_TRUNC;

    #[doc = "Perform all writes to the file at the end of the file.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_APPEND therein"]
    flag FILE_APPEND = O_APPEND;

    #[doc = "Send a signal to the process when the file becomes ready for reading or \
             writing.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_ASYNC therein"]
    flag FILE_SIGNAL_IO = O_ASYNC;

    #[doc = "Ensure that all data has been passed to the hardware after a write.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_SYNC therein"]
    flag FILE_SYNC = O_SYNC;

    #[doc = "Ensure that enough data has been passed to the hardware after a write so \
             that the data can be read back.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_DSYNC therein"]
    flag FILE_DATA_SYNC = O_DSYNC;

    #[doc = "Return an error instead of blocking.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_NONBLOCK therein"]
    flag FILE_DONT_BLOCK = O_NONBLOCK;

    #[doc = "Create a file that can only be used to identify a position in the \
             filesystem.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_PATH therein"]
    flag FILE_PATH = O_PATH;

    #[doc = "Create a temporary file that has no name in the filesystem.\n"]
    #[doc = "= Remarks"]
    #[doc = "The provided path should specify a directory in which the unnamed inode \
             will be created.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_TMPFILE therein"]
    flag FILE_TEMP = O_TMPFILE;

    #[doc = "Allow opening large files on 32 bit systems.\n"]
    #[doc = "= Remarks"]
    #[doc = "The implementation always sets this flag.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_LARGEFILE therein"]
    flag FILE_LARGE = O_LARGEFILE;
}

impl FileFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: FileFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: FileFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: FileFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// The permissions of a file.
#[derive(Pod, Eq)]
pub struct Mode(pub umode_t);

impl Mode {
    /// Create permissions will all bits unset.
    pub fn empty() -> Mode {
        Mode(0)
    }

    /// Create permissions with the default bits for a file:
    ///
    /// - `owner readable`
    /// - `owner writable`
    /// - `group readable`
    /// - `world readable`
    pub fn new_file() -> Mode {
        Mode(0o644)
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
        Mode(0o755)
    }

    /// Checks if the `set user id` bit is set.
    pub fn is_set_user_id(&self) -> bool {
        self.0 & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set user id` flag.
    pub fn set_set_user_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISUID, val);
    }

    /// Checks if the `set group id` bit is set.
    pub fn is_set_group_id(&self) -> bool {
        self.0 & cty::S_ISUID != 0
    }

    /// Sets or unsets the `set group id` flag.
    pub fn set_set_group_id(&mut self, val: bool) {
        self.set_bit(cty::S_ISGID, val);
    }

    /// Checks if the `sticky` bit is set.
    pub fn is_sticky(&self) -> bool {
        self.0 & cty::S_ISVTX != 0
    }

    /// Sets or unsets the `sticky` flag.
    pub fn set_sticky(&mut self, val: bool) {
        self.set_bit(cty::S_ISVTX, val);
    }

    /// Checks if the `owner readable` bit is set.
    pub fn is_owner_readable(&self) -> bool {
        self.0 & cty::S_IRUSR != 0
    }

    /// Sets or unsets the `owner readable` flag.
    pub fn set_owner_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRUSR, val);
    }

    /// Checks if the `owner writable` bit is set.
    pub fn is_owner_writable(&self) -> bool {
        self.0 & cty::S_IWUSR != 0
    }

    /// Sets or unsets the `owner writable` flag.
    pub fn set_owner_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWUSR, val);
    }

    /// Checks if the `owner executable` bit is set.
    pub fn is_owner_executable(&self) -> bool {
        self.0 & cty::S_IXUSR != 0
    }

    /// Sets or unsets the `owner executable` flag.
    pub fn set_owner_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXUSR, val);
    }

    /// Checks if the `group readable` bit is set.
    pub fn is_group_readable(&self) -> bool {
        self.0 & cty::S_IRGRP != 0
    }

    /// Sets or unsets the `group readable` flag.
    pub fn set_group_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IRGRP, val);
    }

    /// Checks if the `group writable` bit is set.
    pub fn is_group_writable(&self) -> bool {
        self.0 & cty::S_IWGRP != 0
    }

    /// Sets or unsets the `group writable` flag.
    pub fn set_group_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWGRP, val);
    }

    /// Checks if the `group executable` bit is set.
    pub fn is_group_executable(&self) -> bool {
        self.0 & cty::S_IXGRP != 0
    }

    /// Sets or unsets the `group executable` flag.
    pub fn set_group_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXGRP, val);
    }

    /// Checks if the `world readable` bit is set.
    pub fn is_world_readable(&self) -> bool {
        self.0 & cty::S_IROTH != 0
    }

    /// Sets or unsets the `world readable` flag.
    pub fn set_world_readable(&mut self, val: bool) {
        self.set_bit(cty::S_IROTH, val);
    }

    /// Checks if the `world writable` bit is set.
    pub fn is_world_writable(&self) -> bool {
        self.0 & cty::S_IWOTH != 0
    }

    /// Sets or unsets the `world writable` flag.
    pub fn set_world_writable(&mut self, val: bool) {
        self.set_bit(cty::S_IWOTH, val);
    }

    /// Checks if the `world executable` bit is set.
    pub fn is_world_executable(&self) -> bool {
        self.0 & cty::S_IXOTH != 0
    }

    /// Sets or unsets the `world executable` flag.
    pub fn set_world_executable(&mut self, val: bool) {
        self.set_bit(cty::S_IXOTH, val);
    }

    fn set_bit(&mut self, bit: umode_t, val: bool) {
        self.0 = (self.0 & !bit) | (bit * val as umode_t);
    }
}

impl Debug for Mode {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        macro_rules! w {
            ($cond:expr, $c:expr) => { try!(if $cond { w.write_str($c) } else { w.write_str("-") }) }
        };
        w!(self.is_owner_readable(), "r");
        w!(self.is_owner_writable(), "w");
        match (self.is_owner_executable(), self.is_set_user_id()) {
            (true,  true)  => try!(w.write_str("s").ignore_ok()),
            (true,  false) => try!(w.write_str("x").ignore_ok()),
            (false, _)     => try!(w.write_str("-").ignore_ok()),
        }
        w!(self.is_group_readable(), "r");
        w!(self.is_group_writable(), "w");
        match (self.is_group_executable(), self.is_set_group_id()) {
            (true,  true)  => try!(w.write_str("s").ignore_ok()),
            (true,  false) => try!(w.write_str("x").ignore_ok()),
            (false, _)     => try!(w.write_str("-").ignore_ok()),
        }
        w!(self.is_world_readable(), "r");
        w!(self.is_world_writable(), "w");
        match (self.is_world_executable(), self.is_sticky()) {
            (true,  true)  => try!(w.write_str("t").ignore_ok()),
            (true,  false) => try!(w.write_str("x").ignore_ok()),
            (false, true)  => try!(w.write_str("T").ignore_ok()),
            (false, false) => try!(w.write_str("-").ignore_ok()),
        }
        Ok(())
    }
}

impl Display for Mode {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self, w)
    }
}

impl Parsable for Mode {
    fn parse_bytes_init(s: &[u8]) -> Result<(Mode, usize)> {
        if s.len() < 9 {
            return Err(error::InvalidSequence);
        }
        let mut mode = Mode(0);
        match s[0] {
            b'r' => mode.set_owner_readable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[1] {
            b'w' => mode.set_owner_writable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[2] {
            b's' => {
                mode.set_owner_executable(true);
                mode.set_set_user_id(true);
            },
            b'x' => mode.set_owner_executable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[3] {
            b'r' => mode.set_group_readable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[4] {
            b'w' => mode.set_group_writable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[5] {
            b's' => {
                mode.set_group_executable(true);
                mode.set_set_group_id(true);
            },
            b'x' => mode.set_group_executable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[6] {
            b'r' => mode.set_world_readable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[7] {
            b'w' => mode.set_world_writable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[8] {
            b't' => {
                mode.set_world_executable(true);
                mode.set_sticky(true);
            },
            b'x' => mode.set_world_executable(true),
            b'T' => mode.set_sticky(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        Ok((mode, 9))
    }
}

/// A way to access a file.
///
/// This type is used to check whether a file can be accessed with a certain set of
/// permissions.
#[derive(Pod, Eq)]
pub struct AccessMode(pub umode_t);

impl AccessMode {
    /// Create an access mode with all bits unset.
    pub fn empty() -> AccessMode {
        AccessMode(0)
    }

    /// Checks if the `readable` bit is set.
    pub fn is_readable(&self) -> bool {
        self.0 & S_IROTH != 0
    }

    /// Sets or unsets the `readable` flag.
    pub fn set_readable(&mut self, val: bool) {
        self.set_bit(S_IROTH, val);
    }

    /// Checks if the `writable` bit is set.
    pub fn is_writable(&self) -> bool {
        self.0 & S_IWOTH != 0
    }

    /// Sets or unsets the `writable` flag.
    pub fn set_writable(&mut self, val: bool) {
        self.set_bit(S_IWOTH, val);
    }

    /// Checks if the `executable` bit is set.
    pub fn is_executable(&self) -> bool {
        self.0 & S_IXOTH != 0
    }

    /// Sets or unsets the `executable` flag.
    pub fn set_executable(&mut self, val: bool) {
        self.set_bit(S_IXOTH, val);
    }

    fn set_bit(&mut self, bit: umode_t, val: bool) {
        self.0 = (self.0 & !bit) | (bit * val as umode_t);
    }
}

impl Debug for AccessMode {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        macro_rules! w {
            ($cond:expr, $c:expr) => { try!(if $cond { w.write_str($c) } else { w.write_str("-") }) }
        };
        w!(self.is_readable(), "r");
        w!(self.is_writable(), "w");
        w!(self.is_executable(), "x");
        Ok(())
    }
}

impl Display for AccessMode {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self, w)
    }
}

impl Parsable for AccessMode {
    fn parse_bytes_init(s: &[u8]) -> Result<(AccessMode, usize)> {
        if s.len() < 3 {
            return Err(error::InvalidSequence);
        }
        let mut mode = AccessMode(0);
        match s[0] {
            b'r' => mode.set_readable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[1] {
            b'w' => mode.set_writable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[2] {
            b'x' => mode.set_executable(true),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        Ok((mode, 3))
    }
}
