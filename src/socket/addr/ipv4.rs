// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{
    BYTES_PER_SHORT, AF_UNIX, AF_INET, AF_INET6, sa_family_t, c_int, sockaddr_in,
    sockaddr_in6,
};

pub struct Ipv4SockAddr { data: [u8] }
