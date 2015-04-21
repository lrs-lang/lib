// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use io::{BufRead};
use buf_reader::{BufReader};
use fmt::{Debug, Write};
use base::error::{self};
use str_one::{AsByteStr, ByteStr};
use str_two::{ByteString};
use cty::alias::{UserId, GroupId};
use parse::{Parse};
use file::{File};
use rmo::{ToOwned};
use iter::{IteratorExt};

use {LineReader};

/// Constant default value for non-allocating user info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

/// Struct holding non-allocated user info.
#[derive(Copy, Eq)]
pub struct Info<'a> {
    name:     &'a ByteStr,
    password: &'a ByteStr,
    user_id:  UserId,
    group_id: GroupId,
    comment:  &'a ByteStr,
    home:     &'a ByteStr,
    shell:    &'a ByteStr,
}

impl<'a> Info<'a> {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(buf: &'a mut [u8], id: UserId) -> Result<Info<'a>> {
        Info::find_by(buf, |user| user.user_id == id)
    }

    /// Retrieves user info of the user with name `name`.
    pub fn from_user_name<S>(buf: &'a mut [u8], name: S) -> Result<Info<'a>>
        where S: AsByteStr,
    {
        let name = name.as_byte_str();
        Info::find_by(buf, |user| user.name == name)
    }

    /// Finds the first user that satisfies the predicate.
    pub fn find_by<F>(buf: &'a mut [u8], pred: F) -> Result<Info<'a>>
        where F: Fn(&Info) -> bool,
    {
        let mut err = Ok(());
        {
            let mut iter = iter_buf(Some(&mut err));
            while let Some(user) = iter.next(buf) {
                if pred(&user) {
                    // the borrow checked doesn't understand that return ends the loop
                    let user = unsafe { mem::cast(user) };
                    return Ok(user);
                }
            }
        }
        try!(err);
        Err(error::DoesNotExist)
    }

    /// Copies the contained data and returns owned information.
    pub fn to_owned(&self) -> Result<Information> {
        Ok(Information {
            name:     try!(self.name.to_owned()),
            password: try!(self.password.to_owned()),
            user_id:  self.user_id,
            group_id: self.group_id,
            comment:  try!(self.comment.to_owned()),
            home:     try!(self.home.to_owned()),
            shell:    try!(self.shell.to_owned()),
        })
    }
}

impl<'a> Debug for Info<'a> {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Info {{ name: {:?}, password: {:?}, user_id: {:?}, group_id: {:?}, \
                    comment: {:?}, home: {:?}, shell: {:?} }}",
                    self.name, self.password, self.user_id, self.group_id, self.comment,
                    self.home, self.shell)
    }
}

/// Struct holding allocated user info.
#[derive(Clone, Eq)]
pub struct Information {
    name:     ByteString,
    password: ByteString,
    user_id:  UserId,
    group_id: GroupId,
    comment:  ByteString,
    home:     ByteString,
    shell:    ByteString,
}

impl Information {
    /// Retrieves user info of the user with id `id`.
    pub fn from_user_id(id: UserId) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_id(&mut buf, id).chain(|i| i.to_owned())
    }

    /// Retrieves user info of the user with name `name`.
    pub fn from_user_name<S>(name: S) -> Result<Information>
        where S: AsByteStr
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_name(&mut buf, name).chain(|i| i.to_owned())
    }

    /// Finds the first user that satisfies the predicate.
    pub fn find_by<F>(pred: F) -> Result<Information>
        where F: Fn(&Info) -> bool,
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).chain(|i| i.to_owned())
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
    fn name(&self)     -> &ByteStr;
    /// Password of the user.
    fn password(&self) -> &ByteStr;
    /// User id of the user.
    fn user_id(&self)  -> UserId;
    /// Group id of the user.
    fn group_id(&self) -> GroupId;
    /// Comment of the user.
    fn comment(&self)  -> &ByteStr;
    /// Home folder of the user.
    fn home(&self)     -> &ByteStr;
    /// Shell of the user.
    fn shell(&self)    -> &ByteStr;
}

impl<'a> UserInfo for Info<'a> {
    fn name(&self)     -> &ByteStr { self.name     }
    fn password(&self) -> &ByteStr { self.password }
    fn user_id(&self)  -> UserId    { self.user_id  }
    fn group_id(&self) -> GroupId   { self.group_id }
    fn comment(&self)  -> &ByteStr { self.comment  }
    fn home(&self)     -> &ByteStr { self.home     }
    fn shell(&self)    -> &ByteStr { self.shell    }
}

impl UserInfo for Information {
    fn name(&self)     -> &ByteStr { &self.name     }
    fn password(&self) -> &ByteStr { &self.password }
    fn user_id(&self)  -> UserId    { self.user_id   }
    fn group_id(&self) -> GroupId   { self.group_id  }
    fn comment(&self)  -> &ByteStr { &self.comment  }
    fn home(&self)     -> &ByteStr { &self.home     }
    fn shell(&self)    -> &ByteStr { &self.shell    }
}

/// Returns an allocating iterator over the users in `/etc/passwd`.
///
/// Errors can optionally be stored in `error`.
pub fn iter<'a>(error: Option<&'a mut Result>) -> InformationIter<'a> {
    InformationIter::new(error)
}

/// An allocating iterator over users.
pub struct InformationIter<'a> {
    file: BufReader<'static, File>,
    err: Option<&'a mut Result>,
}

impl<'a> InformationIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InformationIter<'a> {
        match File::open_read("/etc/passwd") {
            Err(e) => InformationIter::error_dummy(e, error),
            Ok(f) => {
                match BufReader::allocate(f, 1024) {
                    Ok(b) => InformationIter {
                        file: b,
                        err: error,
                    },
                    Err(e) => InformationIter::error_dummy(e, error),
                }
            },
        }
    }

    fn error_dummy(e: error::Errno, error: Option<&'a mut Result>) -> InformationIter<'a> {
        if let Some(err) = error {
            *err = Err(e);
        }
        InformationIter {
            file: BufReader::new(File::invalid(), &mut []),
            err: None,
        }
    }

    fn set_err(&mut self, e: error::Errno) {
        if let Some(ref mut err) = self.err {
            **err = Err(e);
        }
    }
}

impl<'a> Iterator for InformationIter<'a> {
    type Item = Information;

    fn next(&mut self) -> Option<Information> {
        let mut buf = vec!();
        if let Err(e) = self.file.copy_until(&mut buf, b'\n') {
            self.set_err(e);
            None
        } else if buf.len() > 0 {
            let buf = match buf.last() {
                Some(&b'\n') => &buf[..buf.len()-1],
                _ => &buf[..],
            };
            let (parts, uid, gid) = match parse_line(buf) {
                Some(p) => p,
                _ => {
                    self.set_err(error::InvalidSequence);
                    return None;
                }
            };
            Some(Information {
                name:     ByteString::from_vec(parts[0].to_owned().unwrap()),
                password: ByteString::from_vec(parts[1].to_owned().unwrap()),
                user_id:  uid,
                group_id: gid,
                comment:  ByteString::from_vec(parts[4].to_owned().unwrap()),
                home:     ByteString::from_vec(parts[5].to_owned().unwrap()),
                shell:    ByteString::from_vec(parts[6].to_owned().unwrap()),
            })
        } else {
            None
        }
    }
}

/// Returns an non-allocating iterator over the users in `/etc/passwd`.
///
/// Errors can optionally be stored in `error`.
pub fn iter_buf<'a>(error: Option<&'a mut Result>) -> InfoIter<'a> {
    InfoIter::new(error)
}

/// An non-allocating iterator over users.
pub struct InfoIter<'a> {
    reader: LineReader<'a>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InfoIter<'a> {
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
                name:     parts[0].as_byte_str(),
                password: parts[1].as_byte_str(),
                user_id:  uid,
                group_id: gid,
                comment:  parts[4].as_byte_str(),
                home:     parts[5].as_byte_str(),
                shell:    parts[6].as_byte_str(),
            })
        } else {
            self.reader.set_err(error::InvalidSequence);
            None
        }
    }
}

fn parse_line(line: &[u8]) -> Option<([&[u8]; 7], UserId, GroupId)> {
    let mut parts = [&[][..]; 7];
    if line.split(|&c| c == b':').collect_into(&mut parts) < 7 {
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
