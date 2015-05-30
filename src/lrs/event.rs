// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_event::{Eventfd};
pub use lrs_event::flags::{EventfdFlags};

pub mod flags {
    pub use lrs_event::flags::{
        EFD_NONE, EFD_CLOSE_ON_EXEC, EFD_DONT_BLOCK, EFD_SEMAPHORE,
    };
}
