// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

use base::prelude::*;
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};

#[derive(Pod, Eq)]
pub struct ResourceUser(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* usr $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: ResourceUser = ResourceUser(cty::$val);)*

        impl Debug for ResourceUser {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Invalid({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    #[doc = "This process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:getrusage(2) and RUSAGE_SELF therein"]
    usr Process = RUSAGE_SELF;

    #[doc = "The children of this process.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:getrusage(2) and RUSAGE_CHILDREN therein"]
    usr Children = RUSAGE_CHILDREN;

    #[doc = "This thread.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:getrusage(2) and RUSAGE_THREAD therein"]
    usr Thread = RUSAGE_THREAD;
}
