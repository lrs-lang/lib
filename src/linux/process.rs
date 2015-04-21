// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Process handling.

pub use linux_cty::alias::{ProcessId};
pub use linux_process::{this_process_id, parent_process_id};
pub use linux_process::ids::{UserIds, GroupIds, user_drop_privileges,
                             group_drop_privileges, user_set_effective_ids,
                             group_set_effective_ids, num_supplementary_groups,
                             supplementary_groups, set_supplementary_groups};
