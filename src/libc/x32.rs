// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct pthread_t {
    _data: [u32; 1],
}

// XXX: This has size 32 in glibc but 36 in musl. We'll just waste some bytes to be
// compatible with both. Note that this is never embedded in another libc structure.
pub struct pthread_attr_t {
    _data: [u32; 9],
}
