// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use io::{Read, Write};
use vec::{Vec};

pub const BUF_READ_STEP_SIZE: usize = 4096;

pub trait ReadExt : Read {
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let mut len = 0;
        loop {
            let buf_len = buf.len();
            try!(buf.reserve(BUF_READ_STEP_SIZE));
            unsafe { buf.set_len(buf_len + BUF_READ_STEP_SIZE); }
            match self.read_all(&mut buf[buf_len..buf_len+BUF_READ_STEP_SIZE]) {
                Ok(BUF_READ_STEP_SIZE) => len += BUF_READ_STEP_SIZE,
                Ok(n) => {
                    unsafe { buf.set_len(buf_len + n); }
                    len += n;
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(len)
    }
}

impl<T: Read> ReadExt for T { }

pub trait BufRead : Read {
    fn copy_until<W: Write>(&mut self, dst: &mut W, b: u8) -> Result<usize>;
}
