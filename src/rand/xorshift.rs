// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem, slice};
use {Rng};
#[cfg(not(freestanding))] use {kernel, GetUrandom, DevUrandom};
use io::{Read};

#[derive(Pod, Eq)]
pub struct Xorshift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Xorshift {
    #[cfg(not(freestanding))]
    pub fn new() -> Result<Xorshift> {
        if kernel::has_getrandom() {
            GetUrandom.gen()
        } else {
            try!(DevUrandom::new()).gen()
        }
    }

    pub fn seed(seed: [u32; 4]) -> Xorshift {
        Xorshift {
            x: seed[0],
            y: seed[1],
            z: seed[2],
            w: seed[3],
        }
    }
}

impl Rng for Xorshift {
    fn next_u32(&mut self) -> Result<u32> {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        Ok(self.w)
    }
}

impl Read for Xorshift {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        self.read(buf[0])
    }

    #[cfg(target_arch = "arm")]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut t = self.x ^ (self.x << 11);
        let mut x = self.y;
        let mut y = self.z;
        let mut z = self.w;
        let mut w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);

        mem::copy(buf, w.as_ref());
        let base_ptr = buf.as_mut_ptr() as usize;
        let buf = mem::align_for_mut::<u32>(buf);
        let buf = unsafe { slice::from_ptr(buf.as_mut_ptr() as *mut _, buf.len() / 4) };
        for val in &mut *buf {
            t = x ^ (x << 11);
            x = y;
            y = z;
            z = w;
            w = w ^ (w >> 19) ^ t ^ (t >> 8);
            *val = w;
        }

        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;

        Ok((4 * buf.len()) + (buf.as_ptr() as usize - base_ptr))
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.len() < 4 {
            let base: [u8; 4] = unsafe { mem::cast(self.next_u32().unwrap()) };
            if buf.len() > 2 { buf[2] = base[2]; }
            if buf.len() > 1 { buf[1] = base[1]; }
            if buf.len() > 0 { buf[0] = base[0]; }
            return Ok(buf.len());
        }

        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;
        let mut w = self.w;

        let buf = unsafe { slice::from_ptr(buf.as_mut_ptr() as *mut _, buf.len() / 4) };
        for val in &mut *buf {
            let t = x ^ (x << 11);
            x = y;
            y = z;
            z = w;
            w = w ^ (w >> 19) ^ t ^ (t >> 8);
            *val = w;
        }

        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;

        Ok(4 * buf.len())
    }
}
