// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use io::{BufRead};
use alloc::{self, MemPool};
use buf_reader::{BufReader};
use fmt::{Debug, Write};
use base::error::{self};
use str_one::{ByteStr};
use cty::alias::{UserId, GroupId};
use parse::{Parse};
use file::{File};
use vec::{Vec};
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
    /// Retrieves user info of the user with a certain id.
    ///
    /// [argument, buf]
    /// The buffer in which the information will be stored.
    ///
    /// [argument, id]
    /// The id of the user.
    pub fn from_user_id(buf: &'a mut [u8], id: UserId) -> Result<Info<'a>> {
        Info::find_by(buf, |user| user.user_id == id)
    }

    /// Retrieves user info of the user with a certain name.
    ///
    /// [argument, buf]
    /// The buffer in which the information will be stored.
    ///
    /// [argument, name]
    /// The name of the user.
    pub fn from_user_name<S>(buf: &'a mut [u8], name: S) -> Result<Info<'a>>
        where S: AsRef<ByteStr>,
    {
        let name = name.as_ref();
        Info::find_by(buf, |user| user.name == name)
    }

    /// Finds the first user that satisfies a predicate.
    ///
    /// [argument, buf]
    /// The buffer in which the information will be stored.
    ///
    /// [argument, pred]
    /// The predicate.
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
#[derive(Eq)]
pub struct Information<H = alloc::Heap>
    where H: MemPool,
{
    name:     Vec<u8, H>,
    password: Vec<u8, H>,
    user_id:  UserId,
    group_id: GroupId,
    comment:  Vec<u8, H>,
    home:     Vec<u8, H>,
    shell:    Vec<u8, H>,
}

impl<H> Information<H>
    where H: MemPool+OutOf+Copy,
{
    /// Retrieves user info of the user with a certain id.
    ///
    /// [argument, id]
    /// The id of the user.
    pub fn from_user_id(id: UserId) -> Result<Information<H>> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_id(&mut buf, id).chain(|i| i.to_owned())
    }

    /// Retrieves user info of the user with a certain name.
    ///
    /// [argument, name]
    /// The name of the user.
    pub fn from_user_name<S>(name: S) -> Result<Information<H>>
        where S: AsRef<ByteStr>
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_name(&mut buf, name).chain(|i| i.to_owned())
    }

    /// Finds the first user that satisfies the predicate.
    ///
    /// [argument, pred]
    /// The predicate.
    pub fn find_by<F>(pred: F) -> Result<Information<H>>
        where F: Fn(&Info) -> bool,
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).chain(|i| i.to_owned())
    }
}

impl<H> Information<H>
    where H: MemPool+Copy,
{
    /// Retrieves user info of the user with a certain id.
    ///
    /// [argument, id]
    /// The id of the user.
    ///
    /// [argument, pool]
    /// The pool in which the information will be stored.
    pub fn from_user_id_with_pool(id: UserId, pool: H) -> Result<Information<H>> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_id(&mut buf, id).chain(|i| i.try_to_owned_with_pool(pool))
    }

    /// Retrieves user info of the user with a certain name.
    ///
    /// [argument, name]
    /// The name of the user.
    ///
    /// [argument, pool]
    /// The pool in which the information will be stored.
    pub fn from_user_name_with_pool<S>(name: S, pool: H) -> Result<Information<H>>
        where S: AsRef<ByteStr>
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_user_name(&mut buf, name).chain(|i| i.to_owned_with_pool(pool))
    }

    /// Finds the first user that satisfies the predicate.
    ///
    /// [argument, pred]
    /// The predicate.
    ///
    /// [argument, pool]
    /// The pool in which the information will be stored.
    pub fn find_by_with_pool<F>(pred: F, pool: H) -> Result<Information<H>>
        where F: Fn(&Info) -> bool,
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).chain(|i| i.to_owned_with_pool(pool))
    }
}

impl<H> Information<H>
    where H: MemPool,
{
    /// Borrows the information.
    pub fn to_info<'a>(&'a self) -> Info<'a> {
        Info {
                name:     self.name.as_ref(),
                password: self.password.as_ref(),
                user_id:  self.user_id,
                group_id: self.group_id,
                comment:  self.comment.as_ref(),
                home:     self.home.as_ref(),
                shell:    self.shell.as_ref(),
        }
    }
}

impl<H> Debug for Information<H>
    where H: MemPool,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Information {{ name: {:?}, password: {:?}, user_id: {:?}, \
                    group_id: {:?}, comment: {:?}, home: {:?}, shell: {:?} }}",
                    self.name.as_str(), self.password.as_str(), self.user_id,
                    self.group_id, self.comment.as_str(), self.home.as_str(),
                    self.shell.as_str())
    }
}

/// Objects that hold user info.
pub trait UserInfo {
    /// Returns the name of the user.
    fn name(&self)     -> &ByteStr;
    /// Returns the password of the user.
    fn password(&self) -> &ByteStr;
    /// Returns the user id of the user.
    fn user_id(&self)  -> UserId;
    /// Returns the group id of the user.
    fn group_id(&self) -> GroupId;
    /// Returns the comment of the user.
    fn comment(&self)  -> &ByteStr;
    /// Returns the home folder of the user.
    fn home(&self)     -> &ByteStr;
    /// Returns the shell of the user.
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
    fn name(&self)     -> &ByteStr { self.name.as_ref()     }
    fn password(&self) -> &ByteStr { self.password.as_ref() }
    fn user_id(&self)  -> UserId   { self.user_id           }
    fn group_id(&self) -> GroupId  { self.group_id          }
    fn comment(&self)  -> &ByteStr { self.comment.as_ref()  }
    fn home(&self)     -> &ByteStr { self.home.as_ref()     }
    fn shell(&self)    -> &ByteStr { self.shell.as_ref()    }
}

/// Returns an allocating iterator over the users in `/etc/passwd`.
///
/// [argument, error]
/// An optional parameter in which errors that occur during the iteration will be stored.
///
/// = Remarks
///
/// If the error value was supplied, it should be inspected after the end of the loop.
pub fn iter<'a>(error: Option<&'a mut Result>) -> InformationIter<'a> {
    InformationIter::new(error)
}

/// An allocating iterator over users.
pub struct InformationIter<'a> {
    file: BufReader<File>,
    err: Option<&'a mut Result>,
}

impl<'a> InformationIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InformationIter<'a> {
        match File::open_read("/etc/passwd") {
            Err(e) => InformationIter::error_dummy(e, error),
            Ok(f) => {
                match BufReader::new(f, 1024) {
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
            // FIXME
            file: BufReader::new(File::invalid(), 0).unwrap(),
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
        let mut buf: Vec<u8> = Vec::new();
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
                name:     parts[0].try_to().unwrap(),
                password: parts[1].try_to().unwrap(),
                user_id:  uid,
                group_id: gid,
                comment:  parts[4].try_to().unwrap(),
                home:     parts[5].try_to().unwrap(),
                shell:    parts[6].try_to().unwrap(),
            })
        } else {
            None
        }
    }
}

/// Returns an non-allocating iterator over the users in `/etc/passwd`.
///
/// [argument, error]
/// An optional parameter in which errors that occur during the iteration will be stored.
///
/// = Remarks
///
/// If the error value was supplied, it should be inspected after the end of the loop.
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
    /// [argument, buf]
    /// Scratch space for the iterator.
    ///
    /// = Remarks
    ///
    /// The same buffer must be used for each call to `next`, otherwise the function can
    /// abort, return errors, or return nonsense results.
    pub fn next<'b>(&mut self, buf: &'b mut [u8]) -> Option<Info<'b>> {
        let buf = self.reader.fill(buf);
        if buf.len() == 0 {
            return None;
        }
        if let Some((parts, uid, gid)) = parse_line(buf) {
            Some(Info {
                name:     parts[0].as_ref(),
                password: parts[1].as_ref(),
                user_id:  uid,
                group_id: gid,
                comment:  parts[4].as_ref(),
                home:     parts[5].as_ref(),
                shell:    parts[6].as_ref(),
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
