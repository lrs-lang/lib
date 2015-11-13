// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! lrs configuration options.
//!
//! = Remarks
//!
//! The constants in this module allow the user to query the state of various
//! configuration options that were used during the lrs compilation.

pub use lrs_cfg::{NO_LIBC, RETRY, NO_LINK_ARGS, JEMALLOC, DEVICE_PATHS, TRY_ABORT};
