// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{self, mem, fmt, iter};
use std::io::{BufReader, BufRead};
use std::convert::{From};

use core::result::{Result};
use core::errno::{self, Errno};
use core::ext::{IteratorExt2};
use core::string::{LinuxStr, LinuxString, AsLinuxStr};
use core::alias::{GroupId};

use file::{File};

use {LineReader};

/// Constant default value for non-allocating group info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

/// Struct holding non-allocated group info.
#[derive(Copy, Clone, Eq)]
#[allow(raw_pointer_derive)]
pub struct Info<'a> {
    name:     &'a LinuxStr,
    password: &'a LinuxStr,
    id:       GroupId,
    members:  &'a [u8],
}

impl<'a> Info<'a> {
    /// Retrieves group info of the group with id `id`.
    pub fn from_group_id(id: GroupId, buf: &'a mut [u8]) -> Result<Info<'a>> {
        Info::find_by(buf, |group| group.id == id)
    }

    /// Retrieves group info of the group with name `name`.
    pub fn from_group_name<S: AsLinuxStr>(name: S, buf: &'a mut [u8]) -> Result<Info<'a>> {
        let name = name.as_linux_str();
        Info::find_by(buf, |group| group.name == name)
    }

    /// Finds the first group that satisfies the predicate.
    pub fn find_by<F: Fn(&Info) -> bool>(buf: &'a mut [u8], pred: F) -> Result<Info<'a>> {
        let mut err = Ok(());
        {
            let mut iter = iter_buf(Some(&mut err));
            while let Some(group) = iter.next(buf) {
                if pred(&group) {
                    // the borrow checked doesn't understand that return ends the loop
                    let group = unsafe { mem::transmute(group) };
                    return Ok(group);
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
            id:       self.id,
            members:  self.members().map(|v| v.to_linux_string()).collect(),
        }
    }
}

impl<'a> PartialEq for Info<'a> {
    fn eq(&self, other: &Info<'a>) -> bool {
        if self.name != other.name || self.password != other.password ||
                                                                self.id != other.id {
            return false;
        }
        let iter1 =  self.members().map(Some).chain(iter::repeat(None));
        let iter2 = other.members().map(Some).chain(iter::repeat(None));
        for v in iter1.zip(iter2) {
            match v {
                (None, None) => break,
                (None, _) => return false,
                (_, None) => return false,
                (Some(m1), Some(m2)) if m1 != m2 => return false,
                _ => { },
            }
        }
        true
    }
}

impl<'a> fmt::Debug for Info<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Info {{ name: \"{:?}\", password: \"{:?}\", id: {}, members: [",
                    self.name, self.password, self.id));
        for member in self.members() { try!(write!(fmt, "\"{:?}\", ", member)) }
        write!(fmt, "] }}")
    }
}

/// Struct holding allocated group info.
#[derive(Clone, Eq, PartialEq)]
pub struct Information {
    name:     LinuxString,
    password: LinuxString,
    id:       GroupId,
    members:  Vec<LinuxString>,
}

impl Information {
    /// Retrieves group info of the group with id `id`.
    pub fn from_group_id(id: GroupId) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_group_id(id, &mut buf).map(|i| i.to_owned())
    }

    /// Retrieves group info of the group with name `name`.
    pub fn from_group_name<S: AsLinuxStr>(name: S) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_group_name(name, &mut buf).map(|i| i.to_owned())
    }

    /// Finds the first group that satisfies the predicate.
    pub fn find_by<F: Fn(&Info) -> bool>(pred: F) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).map(|i| i.to_owned())
    }
}

impl fmt::Debug for Information {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Information {{ name: \"{:?}\", password: \"{:?}\", id: {}, members: [",
                    self.name, self.password, self.id));
        for member in &self.members { try!(write!(fmt, "\"{:?}\", ", member)) }
        write!(fmt, "] }}")
    }
}

/// Trait for types that hold group info.
pub trait GroupInfo<'a>
    where <Self as GroupInfo<'a>>::Iterator: Iterator<Item=&'a LinuxStr>
{
    type Iterator;

    /// Name of the group.
    fn name(&self)       -> &LinuxStr;
    /// Password of the group.
    fn password(&self)   -> &LinuxStr;
    /// Id of the group.
    fn id(&self)         -> GroupId;
    /// Iterator over the members of the group.
    fn members(&'a self) -> <Self as GroupInfo>::Iterator;
}

impl<'a: 'b, 'b> GroupInfo<'b> for Info<'a> {
    type Iterator = InfoMemberIter<'b>;

    fn name(&self)     -> &LinuxStr { self.name }
    fn password(&self) -> &LinuxStr { self.password }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'b self) -> InfoMemberIter<'b> {
        InfoMemberIter { members: self.members.split(comma_split) }
    }
}

fn comma_split(b: &u8) -> bool { *b == b',' }

/// Iterator over the members in non-allocated group data.
pub struct InfoMemberIter<'a> {
    members: std::slice::Split<'a, u8, fn(&u8) -> bool>,
}

impl<'a> Iterator for InfoMemberIter<'a> {
    type Item = &'a LinuxStr;

    fn next(&mut self) -> Option<&'a LinuxStr> {
        self.members.next().map(|v| v.as_linux_str())
    }
}

impl<'a> GroupInfo<'a> for Information {
    type Iterator = InformationMemberIter<'a>;

    fn name(&self)     -> &LinuxStr { &self.name }
    fn password(&self) -> &LinuxStr { &self.password }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'a self) -> InformationMemberIter<'a> {
        InformationMemberIter { iter: self.members.iter() }
    }
}

/// Iterator over the members in allocated group data.
pub struct InformationMemberIter<'a> {
    iter: std::slice::Iter<'a, LinuxString>,
}

impl<'a> Iterator for InformationMemberIter<'a> {
    type Item = &'a LinuxStr;

    fn next(&mut self) -> Option<&'a LinuxStr> {
        self.iter.next().map(|v| v.as_linux_str())
    }
}

/// Returns an iterator over the groups in `/etc/group`.
///
/// Errors can optionally be stored in `error`.
pub fn iter<'a>(error: Option<&'a mut Result>) -> InformationIter<'a> {
    InformationIter::new(error)
}

pub struct InformationIter<'a> {
    file: BufReader<File>,
    err: Option<&'a mut Result>,
}

impl<'a> InformationIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InformationIter<'a> {
        match File::open_read("/etc/group") {
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
            self.set_err(From::from(e));
            None
        } else if buf.len() > 0 {
            let buf = match buf.last() {
                Some(&b'\n') => &buf[..buf.len()-1],
                _ => &buf[..],
            };
            let parts: Vec<_> = buf.split(|&c| c == b':').collect();
            if parts.len() != 4 {
                self.set_err(errno::ProtocolError);
                None
            } else {
                let id = match parts[2].as_linux_str().parse() {
                    Ok(id) => id,
                    _ => { self.set_err(errno::ProtocolError); return None; },
                };
                let members = parts[3].split(|&c| c == b',')
                                      .map(|s| LinuxString::from_bytes(s)).collect();
                Some(Information {
                    name:     LinuxString::from_bytes(parts[0]),
                    password: LinuxString::from_bytes(parts[1]),
                    id:       id,
                    members:  members,
                })
            }
        } else {
            None
        }
    }
}

/// Returns an non-allocating iterator over the groups in `/etc/group`.
///
/// Errors can optionally be stored in `error`.
pub fn iter_buf<'a>(error: Option<&'a mut Result>) -> InfoIter<'a> {
    InfoIter::new(error)
}

/// An non-allocating iterator over groups.
pub struct InfoIter<'a> {
    reader: LineReader<'a>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InfoIter<'a> {
        InfoIter { reader: LineReader::new("/etc/group", error) }
    }

    /// Reads the next group.
    ///
    /// The same buffer must be used for each call to `next`, otherwise the function can
    /// panic, return errors, or return nonsense results.
    pub fn next<'b>(&mut self, buf: &'b mut [u8]) -> Option<Info<'b>> { 
        let buf = self.reader.fill(buf);
        if buf.len() == 0 {
            return None;
        }
        if let Some((parts, id)) = parse_line(buf) {
            Some(Info {
                name:     parts[0].as_linux_str(),
                password: parts[1].as_linux_str(),
                id:       id,
                members:  parts[3].as_linux_str().as_slice(),
            })
        } else {
            self.reader.set_err(errno::InvalidSequence);
            None
        }
    }
}

fn parse_line(line: &[u8]) -> Option<([&[u8]; 4], GroupId)> {
    let mut parts = [&[][..]; 4];
    if line.split(|&c| c == b':').collect_into(&mut parts) < 4 {
        return None;
    }
    let id = match parts[2].as_linux_str().parse() {
        Ok(id) => id,
        _ => return None,
    };
    Some((parts, id))
}
