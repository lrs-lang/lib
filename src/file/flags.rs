// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Display, Write};
use base::{error};
use parse::{Parsable};
use cty::{
    c_int, umode_t, S_IROTH, S_IWOTH, S_IXOTH, O_CLOEXEC, O_DIRECT, O_DIRECTORY,
    O_EXCL, O_NOATIME, O_NOCTTY, O_NOFOLLOW, O_TRUNC, O_APPEND, O_ASYNC, O_DSYNC,
    O_NONBLOCK, O_SYNC, O_PATH, O_TMPFILE, O_RDWR, O_RDONLY, O_WRONLY, O_LARGEFILE,
    O_CREAT, S_ISUID, S_ISGID, S_ISVTX, S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP,
    S_IXGRP,
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

macro_rules! create_flags {
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

create_flags! {
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
    #[doc = ":link: link:lrs::file::File::link[link]"]
    #[doc = "The provided path should specify a directory in which the unnamed inode \
             will be created.\n"]
    #[doc = "If this flag is set, then one of FILE_WRITE_ONLY and FILE_READ_WRITE also \
             has to be set. Additionally, the FILE_EXCULSIVE flag can be set. If this \
             flag is not set, then the file can later be made visible in the filesystem \
             via the {link} function. The mode of the file will be the mode passed to \
             `open`.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2) and O_TMPFILE therein"]
    #[doc = "* {link}"]
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

impl BitOr for Mode {
    type Output = Mode;
    fn bitor(self, other: Mode) -> Mode {
        Mode(self.0 | other.0)
    }
}

impl BitAnd for Mode {
    type Output = Mode;
    fn bitand(self, other: Mode) -> Mode {
        Mode(self.0 & other.0)
    }
}

impl Not for Mode {
    type Output = Mode;
    fn not(self) -> Mode {
        Mode(!self.0)
    }
}

/// A mode with the default bits for a file:
///
/// * `user readable`
/// * `user writable`
/// * `group readable`
/// * `world readable`
pub const MODE_FILE: Mode = Mode(S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);

/// A mode with the default bits for a directory:
///
/// * `owner readable`
/// * `owner writable`
/// * `owner executable`
/// * `group readable`
/// * `group executable`
/// * `world readable`
/// * `world executable`
pub const MODE_DIRECTORY: Mode = Mode(S_IRUSR | S_IWUSR | S_IXUSR | S_IRGRP | S_IXGRP |
                                      S_IROTH | S_IXOTH);

/// When the file is executed, the effective user id is set to the owner's user id.
pub const MODE_SET_USER_ID: Mode = Mode(S_ISUID);

/// See the remarks.
///
/// = Remarks
///
/// This flag has different meanings when applied to regular files and directories.
///
/// == On regular files
///
/// When the file is executed, the effective group id is set to the owning group's id.
///
/// == On directories
///
/// When a file is created in this directory, the group owner of the file is the same as
/// for this directory. When a directory is created in this directory, it additionally
/// inherits the set-group-id flag.
///
/// = See also
///
/// * link:man:stat(2) and S_ISGID therein.
pub const MODE_SET_GROUP_ID: Mode = Mode(S_ISGID);

/// Files in a directory with this flag can only be deleted by their owners, the owner of
/// the directory, or a privileged process.
///
/// = See also
///
/// * link:man:stat(2) and S_ISVTX therein.
pub const MODE_STICKY: Mode = Mode(S_ISVTX);

/// The owner of the file can read from it.
pub const MODE_USER_READ: Mode = Mode(S_IRUSR);

/// The owner of the file can write to it.
pub const MODE_USER_WRITE: Mode = Mode(S_IWUSR);

/// See the remarks.
///
/// This flag has different meanings when applied to regular files and directories.
///
/// == On regular files
///
/// The owner of this file can execute it.
///
/// == On directories
///
/// The owner of the directory can read the contents of the directory.
pub const MODE_USER_EXEC: Mode = Mode(S_IXUSR);

/// Members of the owning group of the file can read from it.
pub const MODE_GROUP_READ: Mode = Mode(S_IRGRP);

/// Members of the owning group of the file can write to it.
pub const MODE_GROUP_WRITE: Mode = Mode(S_IWGRP);

/// See the remarks.
///
/// This flag has different meanings when applied to regular files and directories.
///
/// == On regular files
///
/// Members of the owning group of this file can execute it.
///
/// == On directories
///
/// Members of the owning group of the directory can read the contents of the directory.
pub const MODE_GROUP_EXEC: Mode = Mode(S_IXGRP);

/// Everyone can read from the file.
pub const MODE_WORLD_READ: Mode = Mode(S_IROTH);

/// Everyone can write to the file.
pub const MODE_WORLD_WRITE: Mode = Mode(S_IWOTH);

/// See the remarks.
///
/// This flag has different meanings when applied to regular files and directories.
///
/// == On regular files
///
/// Everyone can execute the file.
///
/// == On directories
///
/// Everyone can read the contents of the directory.
pub const MODE_WORLD_EXEC: Mode = Mode(S_IXOTH);

impl Mode {
    /// Sets a mode.
    ///
    /// [argument, mode]
    /// The mode to be set.
    pub fn set(&mut self, mode: Mode) {
        self.0 |= mode.0
    }

    /// Clears a mode.
    ///
    /// [argument, mode]
    /// The mode to be cleared.
    pub fn unset(&mut self, mode: Mode) {
        self.0 &= !mode.0
    }

    /// Returns whether a mode is set.
    ///
    /// [argument, mode]
    /// The mode to be checked.
    pub fn is_set(&self, mode: Mode) -> bool {
        self.0 & mode.0 != 0
    }
}

impl Debug for Mode {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        macro_rules! w {
            ($cond:expr, $c:expr) => { try!(if $cond { w.write_str($c) } else { w.write_str("-") }) }
        };
        w!(self.is_set(MODE_USER_READ), "r");
        w!(self.is_set(MODE_USER_WRITE), "w");
        match (self.is_set(MODE_USER_EXEC), self.is_set(MODE_SET_USER_ID)) {
            (true,  true)  => try!(w.write_str("s").ignore_ok()),
            (true,  false) => try!(w.write_str("x").ignore_ok()),
            (false, _)     => try!(w.write_str("-").ignore_ok()),
        }
        w!(self.is_set(MODE_GROUP_READ), "r");
        w!(self.is_set(MODE_GROUP_WRITE), "w");
        match (self.is_set(MODE_GROUP_EXEC), self.is_set(MODE_SET_GROUP_ID)) {
            (true,  true)  => try!(w.write_str("s").ignore_ok()),
            (true,  false) => try!(w.write_str("x").ignore_ok()),
            (false, _)     => try!(w.write_str("-").ignore_ok()),
        }
        w!(self.is_set(MODE_WORLD_READ), "r");
        w!(self.is_set(MODE_WORLD_WRITE), "w");
        match (self.is_set(MODE_WORLD_EXEC), self.is_set(MODE_STICKY)) {
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
            b'r' => mode.set(MODE_USER_READ),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[1] {
            b'w' => mode.set(MODE_USER_WRITE),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[2] {
            b's' => {
                mode.set(MODE_USER_EXEC);
                mode.set(MODE_SET_USER_ID);
            },
            b'x' => mode.set(MODE_USER_EXEC),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[3] {
            b'r' => mode.set(MODE_GROUP_READ),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[4] {
            b'w' => mode.set(MODE_GROUP_WRITE),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[5] {
            b's' => {
                mode.set(MODE_GROUP_EXEC);
                mode.set(MODE_SET_GROUP_ID);
            },
            b'x' => mode.set(MODE_GROUP_EXEC),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[6] {
            b'r' => mode.set(MODE_WORLD_READ),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[7] {
            b'w' => mode.set(MODE_WORLD_WRITE),
            b'-' => { },
            _ => return Err(error::InvalidSequence),
        }
        match s[8] {
            b't' => {
                mode.set(MODE_WORLD_EXEC);
                mode.set(MODE_STICKY);
            },
            b'x' => mode.set(MODE_WORLD_EXEC),
            b'T' => mode.set(MODE_STICKY),
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
        let mut mode = AccessMode(0);
        for &c in s {
            match c {
                b'r' => mode.set_readable(true),
                b'w' => mode.set_writable(true),
                b'x' => mode.set_executable(true),
                _ => return Err(error::InvalidSequence),
            }
        }
        Ok((mode, s.len()))
    }
}
