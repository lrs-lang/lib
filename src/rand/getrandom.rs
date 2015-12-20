// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Rng, syscall, cty};
use io::{Read};
use rv::{retry};

pub struct GetRandom;

impl Rng for GetRandom { }

impl Read for GetRandom {
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize> {
        retry(|| syscall::getrandom(buf[0], cty::GRND_RANDOM)).map(|r| r as usize)
    }
}

pub struct GetUrandom;

impl Rng for GetUrandom { }

impl Read for GetUrandom {
    fn scatter_read(&mut self, buf: &mut [&mut [d8]]) -> Result<usize> {
        retry(|| syscall::getrandom(buf[0], 0)).map(|r| r as usize)
    }
}
