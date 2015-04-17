// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use arch::{cty};

pub type UserId    = cty::uid_t;
pub type GroupId   = cty::gid_t;
pub type DeviceId  = cty::dev_t;
pub type InodeId   = cty::ino_t;
pub type ProcessId = cty::pid_t;
