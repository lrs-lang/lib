use std::{self, mem, ptr, fmt, iter};
use std::ffi::{CStr};
use std::io::{BufReader, BufRead};
use std::os::unix::{OsStrExt};
use std::marker::{PhantomData};
use std::error::{FromError};

use imp::result::{Result};
use imp::cty::{gid_t, c_char, size_t, c_int};
use imp::errno::{self, Errno};
use imp::file::{File};
use imp::rust::{AsStr, AsLinuxStr, ByteSliceExt};

/// Constant default value for non-allocating group info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

pub type GroupId = gid_t;

/// Struct holding non-allocated group info.
#[derive(Copy, Clone, Eq)]
#[allow(raw_pointer_derive)]
pub struct Info<'a> {
    name:     &'a [u8],
    password: &'a [u8],
    id:       gid_t,
    members:  *const *const c_char,
}

impl<'a> Info<'a> {
    ///// Retrieves group info of the group with id `id`.
    //pub fn from_group_id(id: gid_t, buf: &'a [u8]) -> Result<Info<'a>> {
    //    Info::retrieve(|group, result| unsafe {
    //        getgrgid_r(id, group, buf.as_ptr() as *mut c_char, buf.len() as size_t,
    //                   result)
    //    })
    //}

    ///// Retrieves user info of the user with name `name`.
    //pub fn from_group_name<S: AsLinuxStr>(name: S, buf: &'a [u8]) -> Result<Info<'a>> {
    //    let name = name.as_linux_str().to_cstring().unwrap();
    //    Info::retrieve(|group, result| unsafe {
    //        getgrnam_r(name.as_ptr(), group, buf.as_ptr() as *mut c_char,
    //                   buf.len() as size_t, result)
    //    })
    //}

    //fn retrieve<F: FnMut(&mut group, &mut *mut group) -> c_int>(
    //    mut f: F
    //) -> Result<Info<'a>>
    //{
    //    let mut group = unsafe { mem::zeroed() };
    //    let mut result;
    //    loop {
    //        result = ptr::null_mut();
    //        let res = f(&mut group, &mut result);
    //        if res != 0 {
    //            let err = Errno(-res as _);
    //            if !cfg!(feature = "retry") || err != errno::Interrupted {
    //                return Err(err);
    //            }
    //        } else {
    //            return if result.is_null() {
    //                Err(errno::DoesNotExist)
    //            } else {
    //                unsafe {
    //                    Ok(Info {
    //                        name:     CStr::from_ptr(group.gr_name).to_bytes(),
    //                        password: CStr::from_ptr(group.gr_passwd).to_bytes(),
    //                        id:       group.gr_gid,
    //                        members:  mem::transmute(group.gr_mem),
    //                    })
    //                }
    //            };
    //        }
    //    }
    //}

    /// Copies the contained data and returns owned information.
    pub fn to_owned(&self) -> Information {
        Information {
            name:     self.name.to_vec(),
            password: self.password.to_vec(),
            id:       self.id,
            members:  self.members().map(|v|v.to_vec()).collect(),
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        fmt_group_info(self, fmt)
    }
}

/// Struct holding allocated group info.
#[derive(Clone, Eq, PartialEq)]
pub struct Information {
    name:     Vec<u8>,
    password: Vec<u8>,
    id:       gid_t,
    members:  Vec<Vec<u8>>,
}

impl Information {
    ///// Retrieves group info of the group with id `id`.
    //pub fn from_group_id(id: gid_t) -> Result<Information> {
    //    Information::retrieve(|buf| Info::from_group_id(id, buf))
    //}

    ///// Retrieves group info of the group with name `name`.
    //pub fn from_group_name<S: AsLinuxStr>(name: S) -> Result<Information> {
    //    Information::retrieve(|buf| Info::from_group_name(&name, buf))
    //}

    //fn retrieve<'a, F: FnMut(&mut [u8]) -> Result<Info<'a>>>(
    //    mut f: F
    //) -> Result<Information> 
    //{
    //    let mut buf = Vec::with_capacity(128);
    //    loop {
    //        let cap = buf.capacity();
    //        unsafe { buf.set_len(cap); }
    //        match f(&mut buf) {
    //            Ok(info) => return Ok(info.to_owned()),
    //            Err(errno::RangeError) => { },
    //            Err(e) => return Err(e),
    //        }
    //        buf.reserve(cap);
    //    }
    //}
}

impl fmt::Debug for Information {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        fmt_group_info(self, fmt)
    }
}

fn fmt_group_info<'a, T: GroupInfo<'a>>(
    i: &'a T,
    fmt: &mut fmt::Formatter
) -> std::result::Result<(), fmt::Error> {
    try!(write!(fmt, "Info {{ name: \"{}\", password: \"{}\", id: {}, members: [",
                i.name().as_str_lossy(),
                i.password().as_str_lossy(),
                i.id()));
    for member in i.members() {
        try!(write!(fmt, "\"{}\", ", member.as_str_lossy()));
    }
    write!(fmt, "] }}")
}

/// Trait for types that hold group info.
pub trait GroupInfo<'a>
    where <Self as GroupInfo<'a>>::Iterator: Iterator<Item=&'a [u8]>
{
    type Iterator;

    /// Name of the group.
    fn name(&self)       -> &[u8];
    /// Password of the group.
    fn password(&self)   -> &[u8];
    /// Id of the group.
    fn id(&self)         -> GroupId;
    /// Iterator over the members of the group.
    fn members(&'a self) -> <Self as GroupInfo>::Iterator;
}

impl<'a: 'b, 'b> GroupInfo<'b> for Info<'a> {
    type Iterator = InfoMemberIter<'b>;

    fn name(&self)     -> &[u8] { self.name }
    fn password(&self) -> &[u8] { self.password }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'b self) -> InfoMemberIter<'b> {
        InfoMemberIter { members: self.members, marker: PhantomData }
    }
}

/// Iterator over the members in non-allocated group data.
pub struct InfoMemberIter<'a> {
    members: *const *const c_char,
    marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for InfoMemberIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        unsafe {
            if (*self.members).is_null() {
                None
            } else {
                let ret = CStr::from_ptr(*self.members).to_bytes();
                self.members = self.members.offset(1);
                Some(ret)
            }
        }
    }
}

impl<'a> GroupInfo<'a> for Information {
    type Iterator = InformationMemberIter<'a>;

    fn name(&self)     -> &[u8] { &self.name[..] }
    fn password(&self) -> &[u8] { &self.password[..] }
    fn id(&self)       -> GroupId { self.id }

    fn members(&'a self) -> InformationMemberIter<'a> {
        InformationMemberIter { iter: self.members.iter() }
    }
}

/// Iterator over the members in allocated group data.
pub struct InformationMemberIter<'a> {
    iter: std::slice::Iter<'a, Vec<u8>>,
}

impl<'a> Iterator for InformationMemberIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        self.iter.next().map(|v|&v[..])
    }
}


// TODO: So much copy paste from ::user ....

/// Returns an iterator over the groups in `/etc/group`.
///
/// Errors can optionally be stored in `error`.
pub fn iter<'a>(error: Option<&'a mut Result<()>>) -> InfoIter<'a> {
    InfoIter::new(error)
}

pub struct InfoIter<'a> {
    file: BufReader<File>,
    err: Option<&'a mut Result<()>>,
}

impl<'a> InfoIter<'a> {
    fn new(error: Option<&'a mut Result<()>>) -> InfoIter<'a> {
        match File::open_read("/etc/group") {
            Err(e) => {
                if let Some(err) = error {
                    *err = Err(e);
                }
                InfoIter {
                    file: BufReader::with_capacity(0, File::invalid()),
                    err: None,
                }
            },
            Ok(f) => InfoIter {
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

impl<'a> Iterator for InfoIter<'a> {
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
            let parts: Vec<_> = SliceExt::split(buf, |&c| c == b':').collect();
            if parts.len() != 4 {
                self.set_err(errno::ProtocolError);
                None
            } else {
                let id = match parts[2].parse() {
                    Ok(id) => id,
                    _ => { self.set_err(errno::ProtocolError); return None; },
                };
                let members = SliceExt::split(parts[3], |&c| c == b',')
                                        .map(|s| s.to_vec()).collect();
                Some(Information {
                    name:     parts[0].to_vec(),
                    password: parts[1].to_vec(),
                    id:       id,
                    members:  members,
                })
            }
        } else {
            None
        }
    }
}
