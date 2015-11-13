// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_cfg"]
#![crate_type = "lib"]
#![feature(no_std)]
#![no_std]

/// Whether lrs was compiled with the `no_libc` configuration.
pub const NO_LIBC: bool = cfg!(no_libc);

/// Whether lrs was compiled with the `retry` configuration.
pub const RETRY: bool = cfg!(retry);

/// Whether lrs was compiled with the `no_link_args` configuration.
pub const NO_LINK_ARGS: bool = cfg!(no_link_args);

/// Whether lrs was compiled with the `jemalloc` configuration.
pub const JEMALLOC: bool = cfg!(jemalloc);

/// Whether lrs was compiled with the `device_paths` configuration.
pub const DEVICE_PATHS: bool = cfg!(device_paths);

/// Whether lrs was compiled with the `try_abort` configuration.
pub const TRY_ABORT: bool = cfg!(try_abort);
