// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{self, mem, ptr, fmt};
use std::io::{BufReader, BufRead};
use std::ffi::{CStr};
use std::error::{FromError};

use imp::result::{Result};
use imp::cty::{uid_t, gid_t, c_char, size_t, c_int};
use imp::errno::{self};
use imp::rust::{AsStr, AsLinuxStr, ByteSliceExt, IteratorExt2};
use imp::file::{File};
use imp::util::{memchr};

use group::{GroupId};

/// Constant default value for non-allocating user info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

pub type UserId = uid_t;

/// Struct holding non-allocated user info.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info<'a> {
    name:     &'a [u8],
    password: &'a [u8],
    user_id:  UserId,
    group_id: GroupId,
    comment:  &'a [u8],
    home:     &'a [u8],
    shell:    &'a [u8],
}

impl<'a> Info<'a> {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(id: UserId, buf: &'a mut [u8]) -> Result<Info<'a>> {
        Info::find_by(buf, |user| user.user_id == id)
    }

    /// Retrieves user info of the user with name `name`.
    pub fn from_user_name<S: AsLinuxStr>(name: S, buf: &'a mut [u8]) -> Result<Info<'a>> {
        let name = name.as_bytes();
        Info::find_by(buf, |user| user.name == name)
    }

    /// Finds the first user that satisfies the predicate.
    pub fn find_by<F: Fn(&Info) -> bool>(buf: &'a mut [u8], pred: F) -> Result<Info<'a>> {
        let mut err = Ok(());
        {
            let mut iter = iter_buf(Some(&mut err));
            while let Some(user) = iter.next(buf) {
                if pred(&user) {
                    // the borrow checked doesn't understand that return ends the loop
                    let user = unsafe { mem::transmute(user) };
                    return Ok(user);
                }
            }
        }
        try!(err);
        Err(errno::DoesNotExist)
    }

    /// Copies the contained data and returns owned information.
    pub fn to_owned(&self) -> Information {
        Information {
            name:     self.name.to_vec(),
            password: self.password.to_vec(),
            user_id:  self.user_id,
            group_id: self.group_id,
            comment:  self.comment.to_vec(),
            home:     self.home.to_vec(),
            shell:    self.shell.to_vec(),
        }
    }
}

impl<'a> fmt::Debug for Info<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(fmt, "Info {{ name: \"{}\", password: \"{}\", user_id: {}, group_id: {}, \
                     comment: \"{}\", home: \"{}\", shell: \"{}\" }}",
               self.name.as_str_lossy(),
               self.password.as_str_lossy(),
               self.user_id,
               self.group_id,
               self.comment.as_str_lossy(),
               self.home.as_str_lossy(),
               self.shell.as_str_lossy())
    }
}

/// Struct holding allocated user info.
#[derive(Clone, Eq, PartialEq)]
pub struct Information {
    name:     Vec<u8>,
    password: Vec<u8>,
    user_id:  uid_t,
    group_id: gid_t,
    comment:  Vec<u8>,
    home:     Vec<u8>,
    shell:    Vec<u8>,
}

impl Information {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(id: uid_t) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_id(id, &mut buf).map(|i| i.to_owned())
    }

    /// Retrieves user info of the user with name `name`.
    pub fn from_user_name<S: AsLinuxStr>(name: S) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_name(name, &mut buf).map(|i| i.to_owned())
    }

    /// Finds the first user that satisfies the predicate.
    pub fn find_by<F: Fn(&Info) -> bool>(pred: F) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).map(|i| i.to_owned())
    }

    pub fn to_info<'a>(&'a self) -> Info<'a> {
        Info {
            name:     &self.name,
            password: &self.password,
            user_id:  self.user_id,
            group_id: self.group_id,
            comment:  &self.comment,
            home:     &self.home,
            shell:    &self.shell,
        }
    }
}

impl fmt::Debug for Information {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        self.to_info().fmt(fmt)
    }
}

/// Trait for types that hold user info.
pub trait UserInfo {
    /// Name of the user.
    fn name(&self)     -> &[u8];
    /// Password of the user.
    fn password(&self) -> &[u8];
    /// User id of the user.
    fn user_id(&self)  -> UserId;
    /// Group id of the user.
    fn group_id(&self) -> GroupId;
    /// Comment of the user.
    fn comment(&self)  -> &[u8];
    /// Home folder of the user.
    fn home(&self)     -> &[u8];
    /// Shell of the user.
    fn shell(&self)    -> &[u8];
}

impl<'a> UserInfo for Info<'a> {
    fn name(&self)     -> &[u8]   { self.name     }
    fn password(&self) -> &[u8]   { self.password }
    fn user_id(&self)  -> UserId  { self.user_id  }
    fn group_id(&self) -> GroupId { self.group_id }
    fn comment(&self)  -> &[u8]   { self.comment  }
    fn home(&self)     -> &[u8]   { self.home     }
    fn shell(&self)    -> &[u8]   { self.shell    }
}

impl UserInfo for Information {
    fn name(&self)     -> &[u8]   { &self.name     }
    fn password(&self) -> &[u8]   { &self.password }
    fn user_id(&self)  -> UserId  { self.user_id   }
    fn group_id(&self) -> GroupId { self.group_id  }
    fn comment(&self)  -> &[u8]   { &self.comment  }
    fn home(&self)     -> &[u8]   { &self.home     }
    fn shell(&self)    -> &[u8]   { &self.shell    }
}

/// Returns an allocating iterator over the users in `/etc/passwd`.
///
/// Errors can optionally be stored in `error`.
pub fn iter<'a>(error: Option<&'a mut Result<()>>) -> InformationIter<'a> {
    InformationIter::new(error)
}

/// An allocating iterator over users.
pub struct InformationIter<'a> {
    file: BufReader<File>,
    err: Option<&'a mut Result<()>>,
}

impl<'a> InformationIter<'a> {
    fn new(error: Option<&'a mut Result<()>>) -> InformationIter<'a> {
        match File::open_read("/etc/passwd") {
            Err(e) => {
                if let Some(err) = error {
                    *err = Err(e);
                }
                InformationIter {
                    file: BufReader::with_capacity(0, File::invalid()),
                    err: None,
                }
            },
            Ok(f) => InformationIter {
                file: BufReader::new(f),
                err: error,
            },
        }
    }

    fn set_err(&mut self, e: errno::Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }
}

impl<'a> Iterator for InformationIter<'a> {
    type Item = Information;

    fn next(&mut self) -> Option<Information> {
        let mut buf = vec!();
        if let Err(e) = self.file.read_until(b'\n', &mut buf) {
            self.set_err(FromError::from_error(e));
            None
        } else if buf.len() > 0 {
            let buf = match buf.last() {
                Some(&b'\n') => &buf[..buf.len()-1],
                _ => &buf[..],
            };
            let (parts, uid, gid) = match parse_line(buf) {
                Some(p) => p,
                _ => {
                    self.set_err(errno::InvalidSequence);
                    return None;
                }
            };
            Some(Information {
                name:     parts[0].to_vec(),
                password: parts[1].to_vec(),
                user_id:  uid,
                group_id: gid,
                comment:  parts[4].to_vec(),
                home:     parts[5].to_vec(),
                shell:    parts[6].to_vec(),
            })
        } else {
            None
        }
    }
}

/// Returns an non-allocating iterator over the users in `/etc/passwd`.
///
/// Errors can optionally be stored in `error`.
pub fn iter_buf<'a>(error: Option<&'a mut Result<()>>) -> InfoIter<'a> {
    InfoIter::new(error)
}

/// An allocating iterator over users.
pub struct InfoIter<'a> {
    start: usize,
    end: usize,
    file: File,
    err: Option<&'a mut Result<()>>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result<()>>) -> InfoIter<'a> {
        match File::open_read("/etc/passwd") {
            Err(e) => {
                if let Some(err) = error { *err = Err(e); }
                InfoIter {
                    start: 0,
                    end: 0,
                    file: File::invalid(),
                    err: None,
                }
            },
            Ok(f) => InfoIter {
                start: 0,
                end: 0,
                file: f,
                err: error,
            },
        }
    }

    fn set_err(&mut self, e: errno::Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }

    fn fill<'b>(&mut self, buf: &'b mut [u8]) -> &'b [u8] {
        loop {
            {
                // Borrow checked doesn't understand that return ends the loop.
                let cur = unsafe { mem::transmute(&buf[self.start..self.end]) };
                if let Some(pos) = memchr(cur, b'\n') {
                    self.start += pos + 1;
                    return &cur[..pos];
                }
            }
            // No newline in the current buffer.
            // Move it to the left, try to read more, repeat.
            let dst = buf.as_mut_ptr();
            let src = unsafe { dst.offset(self.start as isize) };
            unsafe { ptr::copy(dst, src, self.end - self.start); }
            self.end -= self.start;
            self.start = 0;
            match self.file.read(&mut buf[self.end..]) {
                Err(e) => {
                    // This can be errno::Interrupted but only if the library was compiled
                    // without the 'retry' feature. The user wants to handle it himself.
                    self.set_err(e);
                    return &[];
                },
                Ok(0) => {
                    if self.end == buf.len() {
                        // The buffer is too small for this entry.
                        self.set_err(errno::NoMemory);
                    } else if self.end > self.start {
                        // Not at EOF but the buffer is not empty. The file is corrupted.
                        self.set_err(errno::InvalidSequence);
                    }
                    return &[];
                },
                Ok(n) => self.end += n,
            }
        }
    }

    /// Reads the next user.
    ///
    /// The same buffer must be used for each call to `next`, otherwise the function can
    /// panic, return errors, or return nonsense results.
    pub fn next<'b>(&mut self, buf: &'b mut [u8]) -> Option<Info<'b>> { 
        let buf = self.fill(buf);
        if buf.len() == 0 {
            return None;
        }
        if let Some((parts, uid, gid)) = parse_line(buf) {
            Some(Info {
                name:     parts[0],
                password: parts[1],
                user_id:  uid,
                group_id: gid,
                comment:  parts[4],
                home:     parts[5],
                shell:    parts[6],
            })
        } else {
            self.set_err(errno::InvalidSequence);
            None
        }
    }
}

fn parse_line(line: &[u8]) -> Option<([&[u8]; 7], UserId, GroupId)> {
    let mut parts = [&[][..]; 7];
    if SliceExt::split(line, |&c| c == b':').collect_into(&mut parts) < 7 {
        return None;
    }
    let user_id = match parts[2].parse() {
        Ok(id) => id,
        _ => return None,
    };
    let group_id = match parts[3].parse() {
        Ok(id) => id,
        _ => return None,
    };
    Some((parts, user_id, group_id))
}
