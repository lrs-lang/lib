use std::{self, mem, ptr, fmt};
use std::io::{BufReader, BufRead};
use std::ffi::{CStr};
use std::os::unix::{OsStrExt};
use std::error::{FromError};

use imp::result::{Result};
use imp::cty::{uid_t, gid_t, c_char, size_t, c_int};
use imp::errno::{self};
use imp::rust::{AsStr, AsLinuxStr, ByteSliceExt};
use imp::file::{File};

use group::{GroupId};

/// Constant default value for non-allocating user info buffer size.
pub const INFO_BUF_SIZE: usize = 1024;

pub type UserId = uid_t;

/// Struct holding non-allocated user info.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info<'a> {
    name:     &'a [u8],
    password: &'a [u8],
    user_id:  uid_t,
    group_id: gid_t,
    comment:  &'a [u8],
    home:     &'a [u8],
    shell:    &'a [u8],
}

impl<'a> Info<'a> {
    ///// Retrieves user info of the user with id `id`.
    //pub fn from_user_id(id: uid_t, buf: &'a [u8]) -> Result<Info<'a>> {
    //    Info::retrieve(|passwd, result| unsafe {
    //        getpwuid_r(id, passwd, buf.as_ptr() as *mut c_char, buf.len() as size_t,
    //                   result)
    //    })
    //}

    ///// Retrieves user info of the user with name `name`.
    //pub fn from_user_name<S: AsLinuxStr>(name: S, buf: &'a [u8]) -> Result<Info<'a>> {
    //    let name = name.as_linux_str().to_cstring().unwrap();
    //    Info::retrieve(|passwd, result| unsafe {
    //        getpwnam_r(name.as_ptr(), passwd, buf.as_ptr() as *mut c_char,
    //                   buf.len() as size_t, result)
    //    })
    //}

    //fn retrieve<F: FnMut(&mut passwd, &mut *mut passwd) -> c_int>(
    //    mut f: F
    //) -> Result<Info<'a>>
    //{
    //    let mut passwd = unsafe { mem::zeroed() };
    //    let mut result;
    //    loop {
    //        result = ptr::null_mut();
    //        let res = f(&mut passwd, &mut result);
    //        if res != 0 {
    //            let err = Errno(-res);
    //            if !cfg!(feature = "retry") || err != errno::Interrupted {
    //                return Err(err);
    //            }
    //        } else {
    //            return if result.is_null() {
    //                Err(errno::DoesNotExist)
    //            } else {
    //                unsafe {
    //                    Ok(Info {
    //                        name:     CStr::from_ptr(passwd.pw_name).to_bytes(),
    //                        password: CStr::from_ptr(passwd.pw_passwd).to_bytes(),
    //                        user_id:  passwd.pw_uid,
    //                        group_id: passwd.pw_gid,
    //                        comment:  CStr::from_ptr(passwd.pw_gecos).to_bytes(),
    //                        home:     CStr::from_ptr(passwd.pw_dir).to_bytes(),
    //                        shell:    CStr::from_ptr(passwd.pw_shell).to_bytes(),
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
    ///// Retrieves user info of the user with id `id`.
    //pub fn from_user_id(id: uid_t) -> Result<Information> {
    //    Information::retrieve(|buf| Info::from_user_id(id, buf))
    //}

    ///// Retrieves user info of the user with name `name`.
    //pub fn from_user_name<S: AsLinuxStr>(name: S) -> Result<Information> {
    //    Information::retrieve(|buf| Info::from_user_name(&name, buf))
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
    fn name(&self)     -> &[u8] { self.name }
    fn password(&self) -> &[u8] { self.password }
    fn user_id(&self)  -> UserId { self.user_id }
    fn group_id(&self) -> GroupId { self.group_id }
    fn comment(&self)  -> &[u8] { self.comment }
    fn home(&self)     -> &[u8] { self.home }
    fn shell(&self)    -> &[u8] { self.shell }
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

/// Returns an iterator over the users in `/etc/passwd`.
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
        match File::open_read("/etc/passwd") {
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
            if parts.len() != 7 {
                self.set_err(errno::ProtocolError);
                None
            } else {
                let user_id = match parts[2].parse() {
                    Ok(id) => id,
                    _ => { self.set_err(errno::ProtocolError); return None; },
                };
                let group_id = match parts[3].parse() {
                    Ok(id) => id,
                    _ => { self.set_err(errno::ProtocolError); return None; },
                };
                Some(Information {
                    name:     parts[0].to_vec(),
                    password: parts[1].to_vec(),
                    user_id:  user_id,
                    group_id: group_id,
                    comment:  parts[4].to_vec(),
                    home:     parts[5].to_vec(),
                    shell:    parts[6].to_vec(),
                })
            }
        } else {
            None
        }
    }
}
