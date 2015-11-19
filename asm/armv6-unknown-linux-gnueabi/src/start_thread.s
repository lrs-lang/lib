# function prototype:
#
# extern fn(flags, stack, ptid, tp, ctid, func, arg) -> c_long
#
# r0 = flags
# r1 = stack
# r2 = ptid
# r3 = tp
# [sp,#16] = ctid
# [sp,#20] = func
# [sp,#24] = arg

.global __start_thread
.type __start_thread,%function
__start_thread:
    stmfd sp!,{r4,r5,r6,r7} // save registers

    ldr r4,[sp,#16] // r4 = ctid
    ldr r5,[sp,#20] // r5 = func
    ldr r6,[sp,#24] // r6 = arg
    mov r7,#120     // __NR_clone
    svc #0

    tst r0,r0
    beq 1f // child thread

    ldmfd sp!,{r4,r5,r6,r7}
    bx lr

1:  mov r0,r6
    bx r5 // does not return
