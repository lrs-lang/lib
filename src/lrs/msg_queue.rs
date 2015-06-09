// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_mqueue::{MqAttr, MsgQueue, remove};
pub use lrs_mqueue::flags::{MqFlags};

pub mod flags {
    pub use lrs_mqueue::flags::{MQ_NONE, MQ_DONT_BLOCK};
}
