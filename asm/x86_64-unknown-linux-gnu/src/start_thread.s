# function prototype:
#
# extern fn(flags, stack, ptid, arg, tp, func, ctid) -> c_long
#
# %rdi = flags 
# %rsi = stack
# %rdx = ptid
# %rcx = arg
# %r8 = tp
# %r9 = func
# 8(%rsp) = ctid

.global __start_thread
.type __start_thread,@function
__start_thread:
    sub $8,%rsi
    mov %rcx,(%rsi) # top of child stack = arg

    mov 8(%rsp),%r10 # %r10 = ctid
    mov $56,%rax # __NR_clone
    syscall

    test %eax,%eax
    jnz 1f # parent thread

    pop %rdi # %rdi = arg
    call *%r9 # does not return

1:  ret
