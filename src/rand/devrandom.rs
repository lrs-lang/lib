// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Rng};
use io::{Read};
use file::{File};

pub struct DevRandom(File);

impl DevRandom {
    pub fn new() -> Result<DevRandom> {
        Ok(DevRandom(try!(File::open_read("/dev/random\0"))))
    }
}

impl Rng for DevRandom { }

impl Read for DevRandom {
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize> {
        self.0.scatter_read(buf)
    }
}

pub struct DevUrandom(File);

impl DevUrandom {
    pub fn new() -> Result<DevUrandom> {
        Ok(DevUrandom(try!(File::open_read("/dev/urandom\0"))))
    }
}

impl Rng for DevUrandom { }

impl Read for DevUrandom {
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize> {
        self.0.scatter_read(buf)
    }
}
