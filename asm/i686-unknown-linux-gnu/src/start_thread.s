# function prototype:
#
# extern fn(flags, stack, ptid, arg, ptp, func, ctid) -> c_long
#
# 8(%ebp) = flags
# 12(%ebp) = stack
# 16(%ebp) = ptid
# 20(%ebp) = arg
# 24(%ebp) = ptp
# 28(%ebp) = func
# 32(%ebp) = ctid

.global __start_thread
.type __start_thread,@function
__start_thread:
    push %ebp
    mov %esp,%ebp
    push %ebx
    push %esi
    push %edi

    mov 12(%ebp),%ecx # %ecx = stack
    sub $16,%ecx      # %ecx = stack - 16
    mov 20(%ebp),%edi # %edi = arg
    mov %edi,(%ecx)   # top value on stack = arg

    mov $120,%eax     # __NR_clone
    mov 8(%ebp),%ebx  # %ebx = flags
    mov 16(%ebp),%edx # %edx = ptid
    mov 24(%ebp),%esi # %esi = ptp
    mov 32(%ebp),%edi # %edi = ctid
    mov 28(%ebp),%ebp # %ebp = func
    int $0x80

    test %eax,%eax
    jnz 1f            # parent

    mov %ebp,%eax     # %eax = func
    xor %ebp,%ebp
    call *%eax        # does not return

1:  pop %edi
    pop %esi
    pop %ebx
    pop %ebp
    ret
