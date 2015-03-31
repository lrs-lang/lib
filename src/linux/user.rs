// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! User handling.

pub use linux_core::alias::{UserId};
pub use linux_user_group::user::{Info, InfoIter, Information, InformationIter,
            INFO_BUF_SIZE, UserInfo, iter, iter_buf};
