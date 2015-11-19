# function prototype:
#
# extern fn(stack_base, stack_size, tmp_stack) -> !
#
# x0 = stack_base
# x1 = stack_size
# x2 = tmp_stack

.global __stop_thread
.type __stop_thread,@function
__stop_thread:
    and x2,x2,#-16
    mov sp,x2

    mov x8,#215 // __NR_munmap
    svc #0

    mov x0,#0
    mov x8,#93 // __NR_exit
    svc #0
