// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! System information.

pub use lrs_sys::{cpu_count, StrInfo, NumInfo, get_random, get_random_non_blocking,
                    enable_accounting, set_host_name, set_domain_name};

pub use lrs_kernel::{
    version,
    has_bpf, has_execveat, has_finit_module, has_getrandom, has_kcmp, has_kexec_file_load,
    has_memfd_create, has_process_vm_readv, has_process_vm_writev, has_renameat2,
    has_sched_getattr, has_sched_setattr, has_seccomp, has_o_tmpfile, has_seek_data,
    has_seek_hole, has_falloc_fl_collapse_range, has_falloc_fl_zero_range,
    has_tfd_ioc_set_ticks, has_epollwakeup,
};
