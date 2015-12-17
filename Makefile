.PHONY: all

native_target := $(shell lrsc -V -v | grep host | cut -d' ' -f 2)
target ?= $(native_target)

all: obj/$(target)/libtest.rlib

-include obj/$(target)/test.d
obj/$(target)/libtest.rlib: obj/$(target)/liblrs.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/test/lib.rs

-include obj/$(target)/lrs.d
obj/$(target)/liblrs.rlib: obj/$(target)/liblrs_cfg.rlib obj/$(target)/liblrs_iter.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_hashmap.rlib obj/$(target)/liblrs_r_syscall.rlib obj/$(target)/liblrs_swap.rlib obj/$(target)/liblrs_int.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_buf_reader.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_tree.rlib obj/$(target)/liblrs_ringbuf.rlib obj/$(target)/liblrs_parse.rlib obj/$(target)/liblrs_rc.rlib obj/$(target)/liblrs_env.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_cty_base.rlib obj/$(target)/liblrs_rand.rlib obj/$(target)/liblrs_hash.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_process.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_saturating.rlib obj/$(target)/liblrs_clone.rlib obj/$(target)/liblrs_lock.rlib obj/$(target)/liblrs_socket.rlib obj/$(target)/liblrs_tlalc.rlib obj/$(target)/liblrs_getopt.rlib obj/$(target)/liblrs_kernel.rlib obj/$(target)/liblrs_c_ptr_ptr.rlib obj/$(target)/liblrs_mqueue.rlib obj/$(target)/liblrs_file.rlib obj/$(target)/liblrs_wrapping.rlib obj/$(target)/liblrs_libc.rlib obj/$(target)/liblrs_netlink.rlib obj/$(target)/liblrs_time_ext.rlib obj/$(target)/liblrs_dir.rlib obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_poll.rlib obj/$(target)/liblrs_cell.rlib obj/$(target)/liblrs_rt.rlib obj/$(target)/liblrs_queue.rlib obj/$(target)/liblrs_fs.rlib obj/$(target)/liblrs_sys.rlib obj/$(target)/liblrs_pipe.rlib obj/$(target)/liblrs_inotify.rlib obj/$(target)/liblrs_dev.rlib obj/$(target)/liblrs_tty.rlib obj/$(target)/liblrs_thread.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_signal.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_mem.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_event.rlib obj/$(target)/liblrs_box.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/lrs/lib.rs

-include obj/$(target)/lrs_cfg.d
obj/$(target)/liblrs_cfg.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/cfg/lib.rs

-include obj/$(target)/lrs_core.d
obj/$(target)/liblrs_core.rlib: 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/core/lib.rs

-include obj/$(target)/lrs_iter.d
obj/$(target)/liblrs_iter.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/iter/lib.rs

-include obj/$(target)/lrs_base.d
obj/$(target)/liblrs_base.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_cty_base.rlib obj/$(target)/liblrs_wrapping.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/base/lib.rs

-include obj/$(target)/lrs_cty_base.d
obj/$(target)/liblrs_cty_base.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/cty_base/lib.rs

-include obj/$(target)/lrs_wrapping.d
obj/$(target)/liblrs_wrapping.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/wrapping/lib.rs

-include obj/$(target)/lrs_vec.d
obj/$(target)/liblrs_vec.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_box.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/vec/lib.rs

-include obj/$(target)/lrs_str_one.d
obj/$(target)/liblrs_str_one.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_cty_base.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_parse.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/str_one/lib.rs

-include obj/$(target)/lrs_arch_fns.d
obj/$(target)/liblrs_arch_fns.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_cty_base.rlib obj/$(target)/liblrs_libc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/arch_fns/lib.rs

-include obj/$(target)/lrs_libc.d
obj/$(target)/liblrs_libc.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/libc/lib.rs

-include obj/$(target)/lrs_fmt.d
obj/$(target)/liblrs_fmt.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_io.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/fmt/lib.rs

-include obj/$(target)/lrs_io.d
obj/$(target)/liblrs_io.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_arch_fns.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/io/lib.rs

-include obj/$(target)/lrs_parse.d
obj/$(target)/liblrs_parse.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/parse/lib.rs

-include obj/$(target)/lrs_alloc.d
obj/$(target)/liblrs_alloc.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_libc.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_tlalc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/alloc/lib.rs

-include obj/$(target)/lrs_cty.d
obj/$(target)/liblrs_cty.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/cty/lib.rs

-include obj/$(target)/lrs_syscall.d
obj/$(target)/liblrs_syscall.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_saturating.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_r_syscall.rlib obj/$(target)/liblrs_atomic.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/syscall/lib.rs

-include obj/$(target)/lrs_saturating.d
obj/$(target)/liblrs_saturating.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/saturating/lib.rs

-include obj/$(target)/lrs_r_syscall.d
obj/$(target)/liblrs_r_syscall.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_cty.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/r_syscall/lib.rs

-include obj/$(target)/lrs_atomic.d
obj/$(target)/liblrs_atomic.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cell.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/atomic/lib.rs

-include obj/$(target)/lrs_cell.d
obj/$(target)/liblrs_cell.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/cell/lib.rs

-include obj/$(target)/lrs_tlalc.d
obj/$(target)/liblrs_tlalc.rlib: obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_thread.rlib obj/$(target)/liblrs_arch_fns.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/tlalc/lib.rs

-include obj/$(target)/lrs_thread.d
obj/$(target)/liblrs_thread.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_lock.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_iter.rlib obj/$(target)/liblrs_clone.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_rt.rlib obj/$(target)/liblrs_mem.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_signal.rlib obj/$(target)/liblrs_libc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/thread/lib.rs

-include obj/$(target)/lrs_lock.d
obj/$(target)/liblrs_lock.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cell.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/lock/lib.rs

-include obj/$(target)/lrs_time_base.d
obj/$(target)/liblrs_time_base.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/time_base/lib.rs

-include obj/$(target)/lrs_rv.d
obj/$(target)/liblrs_rv.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_int.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/rv/lib.rs

-include obj/$(target)/lrs_int.d
obj/$(target)/liblrs_int.rlib: obj/$(target)/liblrs_core.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/int/lib.rs

-include obj/$(target)/lrs_fd.d
obj/$(target)/liblrs_fd.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/fd/lib.rs

-include obj/$(target)/lrs_clone.d
obj/$(target)/liblrs_clone.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_r_syscall.rlib obj/$(target)/liblrs_libc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/clone/lib.rs

-include obj/$(target)/lrs_rt.d
obj/$(target)/liblrs_rt.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_r_syscall.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_lock.rlib obj/$(target)/liblrs_libc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/rt/lib.rs

-include obj/$(target)/lrs_mem.d
obj/$(target)/liblrs_mem.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/mem/lib.rs

-include obj/$(target)/lrs_signal.d
obj/$(target)/liblrs_signal.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_time_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/signal/lib.rs

-include obj/$(target)/lrs_box.d
obj/$(target)/liblrs_box.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/box/lib.rs

-include obj/$(target)/lrs_hashmap.d
obj/$(target)/liblrs_hashmap.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_hash.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/hashmap/lib.rs

-include obj/$(target)/lrs_hash.d
obj/$(target)/liblrs_hash.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_wrapping.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/hash/lib.rs

-include obj/$(target)/lrs_swap.d
obj/$(target)/liblrs_swap.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/swap/lib.rs

-include obj/$(target)/lrs_rmo.d
obj/$(target)/liblrs_rmo.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_str_two.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_arch_fns.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/rmo/lib.rs

-include obj/$(target)/lrs_str_two.d
obj/$(target)/liblrs_str_two.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_box.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/str_two/lib.rs

-include obj/$(target)/lrs_buf_reader.d
obj/$(target)/liblrs_buf_reader.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_str_one.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/buf_reader/lib.rs

-include obj/$(target)/lrs_tree.d
obj/$(target)/liblrs_tree.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cell.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/tree/lib.rs

-include obj/$(target)/lrs_ringbuf.d
obj/$(target)/liblrs_ringbuf.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_wrapping.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/ringbuf/lib.rs

-include obj/$(target)/lrs_rc.d
obj/$(target)/liblrs_rc.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_cell.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_atomic.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/rc/lib.rs

-include obj/$(target)/lrs_env.d
obj/$(target)/liblrs_env.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_rt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_str_two.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_vec.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/env/lib.rs

-include obj/$(target)/lrs_rand.d
obj/$(target)/liblrs_rand.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_kernel.rlib obj/$(target)/liblrs_file.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/rand/lib.rs

-include obj/$(target)/lrs_kernel.d
obj/$(target)/liblrs_kernel.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_parse.rlib obj/$(target)/liblrs_arch_fns.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/kernel/lib.rs

-include obj/$(target)/lrs_file.d
obj/$(target)/liblrs_file.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_int.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_parse.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_dev.rlib obj/$(target)/liblrs_fs.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_cell.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/file/lib.rs

-include obj/$(target)/lrs_dev.d
obj/$(target)/liblrs_dev.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_cty.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/dev/lib.rs

-include obj/$(target)/lrs_fs.d
obj/$(target)/liblrs_fs.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/fs/lib.rs

-include obj/$(target)/lrs_process.d
obj/$(target)/liblrs_process.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib obj/$(target)/liblrs_c_ptr_ptr.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_rt.rlib obj/$(target)/liblrs_env.rlib obj/$(target)/liblrs_file.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_time_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/process/lib.rs

-include obj/$(target)/lrs_c_ptr_ptr.d
obj/$(target)/liblrs_c_ptr_ptr.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty_base.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_alloc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/c_ptr_ptr/lib.rs

-include obj/$(target)/lrs_socket.d
obj/$(target)/liblrs_socket.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_saturating.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_alloc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/socket/lib.rs

-include obj/$(target)/lrs_getopt.d
obj/$(target)/liblrs_getopt.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_str_one.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/getopt/lib.rs

-include obj/$(target)/lrs_mqueue.d
obj/$(target)/liblrs_mqueue.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_file.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/mqueue/lib.rs

-include obj/$(target)/lrs_netlink.d
obj/$(target)/liblrs_netlink.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/netlink/lib.rs

-include obj/$(target)/lrs_time_ext.d
obj/$(target)/liblrs_time_ext.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_file.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/time_ext/lib.rs

-include obj/$(target)/lrs_dir.d
obj/$(target)/liblrs_dir.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_str_two.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_file.rlib obj/$(target)/liblrs_vec.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_alloc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/dir/lib.rs

-include obj/$(target)/lrs_poll.d
obj/$(target)/liblrs_poll.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_saturating.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_time_base.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/poll/lib.rs

-include obj/$(target)/lrs_queue.d
obj/$(target)/liblrs_queue.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cell.rlib obj/$(target)/liblrs_arch_fns.rlib obj/$(target)/liblrs_atomic.rlib obj/$(target)/liblrs_lock.rlib obj/$(target)/liblrs_alloc.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/queue/lib.rs

-include obj/$(target)/lrs_sys.d
obj/$(target)/liblrs_sys.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_time_base.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/sys/lib.rs

-include obj/$(target)/lrs_pipe.d
obj/$(target)/liblrs_pipe.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_saturating.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/pipe/lib.rs

-include obj/$(target)/lrs_inotify.d
obj/$(target)/liblrs_inotify.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_rv.rlib obj/$(target)/liblrs_io.rlib obj/$(target)/liblrs_str_one.rlib obj/$(target)/liblrs_alloc.rlib obj/$(target)/liblrs_rmo.rlib obj/$(target)/liblrs_str_two.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/inotify/lib.rs

-include obj/$(target)/lrs_tty.d
obj/$(target)/liblrs_tty.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_file.rlib obj/$(target)/liblrs_signal.rlib obj/$(target)/liblrs_dev.rlib obj/$(target)/liblrs_fmt.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/tty/lib.rs

-include obj/$(target)/lrs_event.d
obj/$(target)/liblrs_event.rlib: obj/$(target)/liblrs_core.rlib obj/$(target)/liblrs_base.rlib obj/$(target)/liblrs_cty.rlib obj/$(target)/liblrs_fmt.rlib obj/$(target)/liblrs_syscall.rlib obj/$(target)/liblrs_fd.rlib obj/$(target)/liblrs_io.rlib 
	lrsc --emit=link,dep-info --out-dir obj/$(target) --target $(target) src/event/lib.rs

