// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct pthread_t {
    _data: [u64; 1],
}

pub struct pthread_attr_t {
    _data: [u64; 8], // 7 with musl, 8 with glibc
}
