# function prototype:
#
# extern fn(stack_base, stack_size, tmp_stack) -> !
#
# r0 = stack_base
# r1 = stack_size
# r2 = tmp_stack

.global __stop_thread
.type __stop_thread,%function
__stop_thread:
    and r2,r2,#-16
    mov sp,r2

    mov r7,#91 // __NR_munmap
    svc #0

    mov r0,#0
    mov r7,#1 // __NR_exit
    svc #0
