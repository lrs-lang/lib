# function prototype:
#
# extern fn(flags, stack, ptid, tp, ctid, func, arg) -> c_long
#
# x0 = flags
# x1 = stack
# x2 = ptid
# x3 = tp
# x4 = ctid
# x5 = func
# x6 = arg

.global __start_thread
.type __start_thread,@function
__start_thread:
    stp x5,x6,[x1,#-16]! // push func and arg on stack

    mov x8,#220 // __NR_clone
    svc #0

    cbz x0, 1f // child thread
    ret

1:  ldp x1,x0,[sp],#16 // load func and arg from stack
    blr x1
