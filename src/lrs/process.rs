// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Process handling.

pub use lrs_cty::alias::{ProcessId};
pub use lrs_process::{this_process_id, parent_process_id, exit};
pub use lrs_process::exec::{exec};
pub use lrs_process::wait::{
    ChildStatus, WaitFlags, WAIT_EXITED, WAIT_STOPPED, WAIT_CONTINUED, WAIT_NON_BLOCKING,
    WAIT_DONT_REAP, wait_all, wait_id,
};
pub use lrs_process::ids::{UserIds, GroupIds, user_drop_privileges,
                             group_drop_privileges, user_set_effective_ids,
                             group_set_effective_ids, num_supplementary_groups,
                             supplementary_groups, set_supplementary_groups};
pub use lrs_clone::{fork};
