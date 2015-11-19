# function prototype:
#
# extern fn(stack_base, stack_size, tmp_stack) -> !
#
# %rdi = stack_base
# %rsi = stack_size
# %rdx = tmp_stack

.global __stop_thread
.type __stop_thread,@function
__stop_thread:
    mov %rdx,%rsp # kernel might want to use our stack a bit
    and $-16,%rsp

    mov $11,%rax # __NR_munmap
    syscall

    xor %rdi,%rdi
    mov $60,%rax # __NR_exit
    syscall
