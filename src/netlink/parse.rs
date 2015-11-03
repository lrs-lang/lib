// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::{error};
use {MsgHeader};

pub struct MsgIter<'a> {
    data: &'a [u8],
    err: Option<&'a mut Result>,
}

impl<'a> MsgIter<'a> {
    pub fn new(data: &'a [u8], err: Option<&'a mut Result>) -> MsgIter<'a> {
        let mut iter = MsgIter { data: data, err: err };
        if data.as_ptr() as usize & 3 != 0 {
            iter.set_error(error::InvalidArgument);
            iter.data = &[];
        }
        iter
    }

    fn set_error(&mut self, error: error::Errno) {
        if let Some(ref mut e) = self.err {
            **e = Err(error);
        }
    }

    pub fn error(&self) -> Result {
        self.err.as_ref().map(|e| **e).unwrap_or(Ok(()))
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl<'a> Iterator for MsgIter<'a> {
    type Item = (&'a MsgHeader, &'a [u8]);
    fn next(&mut self) -> Option<(&'a MsgHeader, &'a [u8])> {
        let header = match mem::from_bytes::<MsgHeader>(self.data) {
            Some(h) => h,
            _ => return None,
        };
        let len = header.len as usize;
        if len > self.data.len() || len < mem::size_of::<MsgHeader>() {
            self.set_error(error::InvalidSequence);
            return None;
        }
        let payload = &self.data[mem::size_of::<MsgHeader>()..len];
        self.data = mem::align_for::<u32>(&self.data[len..]);
        Some((header, payload))
    }
}

pub struct MsgParser<'a> {
    data: &'a [u8],
}

impl<'a> MsgParser<'a> {
    pub fn new(buf: &'a [u8]) -> Result<MsgParser<'a>> {
        if buf.as_ptr() as usize & 3 != 0 {
            Err(error::InvalidArgument)
        } else {
            Ok(MsgParser { data: buf })
        }
    }

    pub fn peek_raw<T>(&self) -> Result<&T>
        where T: Pod,
    {
        match mem::from_bytes(self.data) {
            Some(v) => Ok(v),
            _ => Err(error::InvalidArgument),
        }
    }
}
