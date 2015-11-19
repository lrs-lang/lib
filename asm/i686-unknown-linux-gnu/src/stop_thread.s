# function prototype:
#
# extern fn(stack_base, stack_size, tmp_stack) -> !
#
# 4(%esp) = stack_base
# 8(%esp) = stack_size
# 12(%esp) = tmp_stack

.global __stop_thread
.type __stop_thread,@function
__stop_thread:
    mov 4(%esp),%ebx # %ebx = stack_base
    mov 8(%esp),%ecx # %ecx = stack_size

    mov 12(%esp),%esp # %esp = tmp_stack
    and $-16,%esp

    mov $91,%eax # __NR_munmap
    int $0x80

    xor %ebx,%ebx
    mov $1,%eax # __NR_exit
    int $0x80
