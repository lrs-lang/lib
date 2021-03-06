// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, slice};
use io::{BufRead};
use buf_reader::{BufReader};
use fmt::{Debug, Write};
use base::error::{self};
use str_one::{ByteStr};
use cty::alias::{GroupId};
use parse::{Parse};
use file::{File};
use vec::{Vec};
use iter::{IteratorExt};

use {LineReader};

/// Constant default value for non-allocating group info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

/// Struct holding non-allocated group info.
#[derive(Copy)]
pub struct Info<'a> {
    name:     &'a ByteStr,
    password: &'a ByteStr,
    id:       GroupId,
    members:  &'a [u8],
}

impl<'a> Info<'a> {
    /// Retrieves group info of the group with a certain id.
    ///
    /// [argument, buf]
    /// The buffer in which the information will be stored.
    ///
    /// [argument, id]
    /// The group id.
    pub fn from_group_id(buf: &'a mut [u8], id: GroupId) -> Result<Info<'a>> {
        Info::find_by(buf, |group| group.id == id)
    }

    /// Retrieves group info of the group with a certain name.
    ///
    /// [argument, buf]
    /// The buffer in which the information will be stored.
    ///
    /// [argument, name]
    /// The group name.
    pub fn from_group_name<S>(buf: &'a mut [u8], name: S) -> Result<Info<'a>>
        where S: AsRef<ByteStr>,
    {
        let name = name.as_ref();
        Info::find_by(buf, |group| group.name == name)
    }

    /// Finds the first group that satisfies a predicate.
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
            while let Some(group) = iter.next(buf) {
                if pred(&group) {
                    // the borrow checked doesn't understand that return ends the loop
                    let group = unsafe { mem::cast(group) };
                    return Ok(group);
                }
            }
        }
        try!(err);
        Err(error::DoesNotExist)
    }

    /// Copies the contained data and returns owned information.
    pub fn to_owned(&self) -> Result<Information> {
        let mut members = Vec::new();
        for member in self.members() {
            let member = try!(member.try_to());
            try!(members.reserve(1));
            members.push(member);
        }
        Ok(Information {
            name:     try!(self.name.try_to()),
            password: try!(self.password.try_to()),
            id:       self.id,
            members:  members,
        })
    }
}

// Not yet sure if we want `chain` and I don't want to implement this in another way right
// now.
//impl<'a> Eq for Info<'a> {
//    fn eq(&self, other: &Info<'a>) -> bool {
//        if self.name != other.name || self.password != other.password ||
//                                                                self.id != other.id {
//            return false;
//        }
//        let iter1 =  self.members().map(Some).chain(iter::repeat(None));
//        let iter2 = other.members().map(Some).chain(iter::repeat(None));
//        for v in iter1.zip(iter2) {
//            match v {
//                (None, None) => break,
//                (None, _) => return false,
//                (_, None) => return false,
//                (Some(m1), Some(m2)) if m1 != m2 => return false,
//                _ => { },
//            }
//        }
//        true
//    }
//}

impl<'a> Debug for Info<'a> {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(write!(w, "Info {{ name: \"{:?}\", password: \"{:?}\", id: {}, members: [",
                    self.name, self.password, self.id));
        for member in self.members() { try!(write!(w, "\"{:?}\", ", member)) }
        write!(w, "] }}")
    }
}

/// Struct holding allocated group info.
#[derive(TryTo, Eq)]
pub struct Information {
    name:     Vec<u8>,
    password: Vec<u8>,
    id:       GroupId,
    members:  Vec<Vec<u8>>,
}

impl Information {
    /// Retrieves group info of the group with a certain id.
    ///
    /// [argument, id]
    /// The group id.
    pub fn from_group_id(id: GroupId) -> Result<Information> {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_group_id(&mut buf, id).chain(|i| i.to_owned())
    }

    /// Retrieves group info of the group with a certain name.
    ///
    /// [argument, name]
    /// The group name.
    pub fn from_group_name<S>(name: S) -> Result<Information>
        where S: AsRef<ByteStr>,
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::from_group_name(&mut buf, name).chain(|i| i.to_owned())
    }

    /// Finds the first group that satisfies a predicate.
    ///
    /// [argument, pred]
    /// The predicate.
    pub fn find_by<F>(pred: F) -> Result<Information>
        where F: Fn(&Info) -> bool,
    {
        let mut buf = [0; INFO_BUF_SIZE];
        Info::find_by(&mut buf, pred).chain(|i| i.to_owned())
    }
}

impl Debug for Information {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(write!(w, "Information {{ name: \"{:?}\", password: \"{:?}\", id: {}, members: [",
                    self.name, self.password, self.id));
        for member in &self.members { try!(write!(w, "\"{:?}\", ", member)) }
        write!(w, "] }}")
    }
}

/// Trait for types that hold group info.
pub trait GroupInfo<'a>
    where <Self as GroupInfo<'a>>::Iterator: Iterator<Item=&'a ByteStr>
{
    type Iterator;

    /// Returns the name of the group.
    fn name(&self)       -> &ByteStr;
    /// Returns the password of the group.
    fn password(&self)   -> &ByteStr;
    /// Returns the id of the group.
    fn id(&self)         -> GroupId;
    /// Returns an iterator over the members of the group.
    fn members(&'a self) -> <Self as GroupInfo>::Iterator;
}

impl<'a: 'b, 'b> GroupInfo<'b> for Info<'a> {
    type Iterator = InfoMemberIter<'b>;

    fn name(&self)     -> &ByteStr { self.name }
    fn password(&self) -> &ByteStr { self.password }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'b self) -> InfoMemberIter<'b> {
        InfoMemberIter { members: self.members.split(comma_split) }
    }
}

fn comma_split(b: &u8) -> bool { *b == b',' }

/// Iterator over the members in non-allocated group data.
pub struct InfoMemberIter<'a> {
    members: slice::Split<'a, u8, fn(&u8) -> bool>,
}

impl<'a> Iterator for InfoMemberIter<'a> {
    type Item = &'a ByteStr;

    fn next(&mut self) -> Option<&'a ByteStr> {
        self.members.next().map(|v| v.as_ref())
    }
}

impl<'a> GroupInfo<'a> for Information {
    type Iterator = InformationMemberIter<'a>;

    fn name(&self)     -> &ByteStr { self.name.as_str() }
    fn password(&self) -> &ByteStr { self.password.as_str() }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'a self) -> InformationMemberIter<'a> {
        InformationMemberIter { iter: self.members.iter() }
    }
}

/// Iterator over the members in allocated group data.
pub struct InformationMemberIter<'a> {
    iter: slice::Items<'a, Vec<u8>>,
}

impl<'a> Iterator for InformationMemberIter<'a> {
    type Item = &'a ByteStr;

    fn next(&mut self) -> Option<&'a ByteStr> {
        self.iter.next().map(|v| v.as_ref())
    }
}

/// Returns an iterator over the groups in `/etc/group`.
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

pub struct InformationIter<'a> {
    file: BufReader<File>,
    err: Option<&'a mut Result>,
}

impl<'a> InformationIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InformationIter<'a> {
        match File::open_read("/etc/group") {
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
            let parts = buf.split(|&c| c == b':').collect();
            if parts.len() != 4 {
                self.set_err(error::ProtocolError);
                None
            } else {
                let id = match parts[2].parse() {
                    Ok(id) => id,
                    _ => { self.set_err(error::ProtocolError); return None; },
                };
                let mut members = Vec::new();
                for member in parts[3].split(|&c| c == b',') {
                    match members.reserve(1).chain(|_| member.try_to()) {
                        Ok(m) => members.push(m),
                        Err(e) => {
                            self.set_err(e);
                            return None;
                        }
                    }
                }
                let name = match parts[0].try_to() {
                    Ok(n) => n,
                    Err(e) => {
                        self.set_err(e);
                        return None;
                    }
                };
                let password = match parts[0].try_to() {
                    Ok(p) => p,
                    Err(e) => {
                        self.set_err(e);
                        return None;
                    }
                };
                Some(Information {
                    name:     name,
                    password: password,
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
/// [argument, error]
/// An optional parameter in which errors that occur during the iteration will be stored.
///
/// = Remarks
///
/// If the error value was supplied, it should be inspected after the end of the loop.
pub fn iter_buf<'a>(error: Option<&'a mut Result>) -> InfoIter<'a> {
    InfoIter::new(error)
}

/// A non-allocating iterator over groups.
pub struct InfoIter<'a> {
    reader: LineReader<'a>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result>) -> InfoIter<'a> {
        InfoIter { reader: LineReader::new("/etc/group", error) }
    }

    /// Reads the next group.
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
        if let Some((parts, id)) = parse_line(buf) {
            Some(Info {
                name:     parts[0].as_ref(),
                password: parts[1].as_ref(),
                id:       id,
                members:  parts[3],
            })
        } else {
            self.reader.set_err(error::InvalidSequence);
            None
        }
    }
}

fn parse_line(line: &[u8]) -> Option<([&[u8]; 4], GroupId)> {
    let mut parts = [&[][..]; 4];
    if line.split(|&c| c == b':').collect_into(&mut parts) < 4 {
        return None;
    }
    let id = match parts[2].parse() {
        Ok(id) => id,
        _ => return None,
    };
    Some((parts, id))
}
