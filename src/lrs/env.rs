// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Process environment.

pub use lrs_rt::{args, arg_count, env};
pub use lrs_env::{var, path, cwd, set_cwd};
