// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use io::{Read, Write};
use vec::{Vec};

pub trait BufRead : Read {
    fn copy_until<W: Write>(&mut self, dst: &mut W, b: u8) -> Result<usize>;
}
