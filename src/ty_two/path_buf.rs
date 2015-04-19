// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use rmo::{ToOwned, AsRef, AsMut};
use ty_one::path::{Path, AsPath, AsMutPath};
use vec::{Vec};
use fmt::{Debug, Write};

pub struct PathBuf {
    data: Vec<u8>,
}

impl ToOwned for Path {
    type Owned = PathBuf;
    fn to_owned(&self) -> Result<PathBuf> {
        Ok(PathBuf { data: try!(self.as_ref().to_owned()) })
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        unsafe { Path::from_bytes_unchecked(&self.data) }
    }
}

impl AsMut<Path> for PathBuf {
    fn as_mut(&mut self) -> &mut Path {
        unsafe { Path::from_bytes_unchecked_mut(&mut self.data) }
    }
}

impl AsPath for PathBuf {
    fn as_path(&self) -> Result<&Path> {
        unsafe { Ok(Path::from_bytes_unchecked(&self.data)) }
    }
}

impl AsMutPath for PathBuf {
    fn as_mut_path(&mut self) -> Result<&mut Path> {
        unsafe { Ok(Path::from_bytes_unchecked_mut(&mut self.data)) }
    }
}

impl Debug for PathBuf {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.as_ref().fmt(w)
    }
}
