// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};
use std::io::{BufReader, BufRead};
use std::error::{FromError};

use core::result::{Result};
use core::cty::{uid_t};
use core::errno::{self};
use core::ext::{IteratorExt2};
use core::string::{AsLinuxStr, LinuxStr, LinuxString};

use file::{File};

use {LineReader};

use group::{GroupId};

/// Constant default value for non-allocating user info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

pub type UserId = uid_t;

/// Struct holding non-allocated user info.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Info<'a> {
    name:     &'a LinuxStr,
    password: &'a LinuxStr,
    user_id:  UserId,
    group_id: GroupId,
    comment:  &'a LinuxStr,
    home:     &'a LinuxStr,
    shell:    &'a LinuxStr,
}

impl<'a> Info<'a> {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(id: UserId, buf: &'a mut [u8]) -> Result<Info<'a>> {
        Info::find_by(buf, |user| user.user_id == id)
    }

    /// Retrieves user info of the user with name `name`.
    pub fn from_user_name<S: AsLinuxStr>(name: S, buf: &'a mut [u8]) -> Result<Info<'a>> {
        let name = name.as_linux_str();
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
            name:     self.name.to_linux_string(),
            password: self.password.to_linux_string(),
            user_id:  self.user_id,
            group_id: self.group_id,
            comment:  self.comment.to_linux_string(),
            home:     self.home.to_linux_string(),
            shell:    self.shell.to_linux_string(),
        }
    }
}

/// Struct holding allocated user info.
#[derive(Clone, Eq, PartialEq)]
pub struct Information {
    name:     LinuxString,
    password: LinuxString,
    user_id:  UserId,
    group_id: GroupId,
    comment:  LinuxString,
    home:     LinuxString,
    shell:    LinuxString,
}

impl Information {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(id: UserId) -> Result<Information> {
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

/// Trait for types that hold user info.
pub trait UserInfo {
    /// Name of the user.
    fn name(&self)     -> &LinuxStr;
    /// Password of the user.
    fn password(&self) -> &LinuxStr;
    /// User id of the user.
    fn user_id(&self)  -> UserId;
    /// Group id of the user.
    fn group_id(&self) -> GroupId;
    /// Comment of the user.
    fn comment(&self)  -> &LinuxStr;
    /// Home folder of the user.
    fn home(&self)     -> &LinuxStr;
    /// Shell of the user.
    fn shell(&self)    -> &LinuxStr;
}

impl<'a> UserInfo for Info<'a> {
    fn name(&self)     -> &LinuxStr { self.name     }
    fn password(&self) -> &LinuxStr { self.password }
    fn user_id(&self)  -> UserId    { self.user_id  }
    fn group_id(&self) -> GroupId   { self.group_id }
    fn comment(&self)  -> &LinuxStr { self.comment  }
    fn home(&self)     -> &LinuxStr { self.home     }
    fn shell(&self)    -> &LinuxStr { self.shell    }
}

impl UserInfo for Information {
    fn name(&self)     -> &LinuxStr { &self.name     }
    fn password(&self) -> &LinuxStr { &self.password }
    fn user_id(&self)  -> UserId    { self.user_id   }
    fn group_id(&self) -> GroupId   { self.group_id  }
    fn comment(&self)  -> &LinuxStr { &self.comment  }
    fn home(&self)     -> &LinuxStr { &self.home     }
    fn shell(&self)    -> &LinuxStr { &self.shell    }
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
                name:     LinuxString::from_bytes(parts[0]),
                password: LinuxString::from_bytes(parts[1]),
                user_id:  uid,
                group_id: gid,
                comment:  LinuxString::from_bytes(parts[4]),
                home:     LinuxString::from_bytes(parts[5]),
                shell:    LinuxString::from_bytes(parts[6]),
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

/// An non-allocating iterator over users.
pub struct InfoIter<'a> {
    reader: LineReader<'a>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result<()>>) -> InfoIter<'a> {
        InfoIter { reader: LineReader::new("/etc/passwd", error) }
    }

    /// Reads the next user.
    ///
    /// The same buffer must be used for each call to `next`, otherwise the function can
    /// panic, return errors, or return nonsense results.
    pub fn next<'b>(&mut self, buf: &'b mut [u8]) -> Option<Info<'b>> { 
        let buf = self.reader.fill(buf);
        if buf.len() == 0 {
            return None;
        }
        if let Some((parts, uid, gid)) = parse_line(buf) {
            Some(Info {
                name:     parts[0].as_linux_str(),
                password: parts[1].as_linux_str(),
                user_id:  uid,
                group_id: gid,
                comment:  parts[4].as_linux_str(),
                home:     parts[5].as_linux_str(),
                shell:    parts[6].as_linux_str(),
            })
        } else {
            self.reader.set_err(errno::InvalidSequence);
            None
        }
    }
}

fn parse_line(line: &[u8]) -> Option<([&[u8]; 7], UserId, GroupId)> {
    let mut parts = [&[][..]; 7];
    if line.split(|&c| c == b':').collect_into(&mut parts) < 7 {
        return None;
    }
    let user_id = match parts[2].as_linux_str().parse() {
        Ok(id) => id,
        _ => return None,
    };
    let group_id = match parts[3].as_linux_str().parse() {
        Ok(id) => id,
        _ => return None,
    };
    Some((parts, user_id, group_id))
}
