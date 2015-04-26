// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_sys::{cpu_count, StrInfo, NumInfo, get_random, get_random_non_blocking,
                    enable_accounting, set_host_name, set_domain_name};
