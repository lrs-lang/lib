.global lrs_restore
.type lrs_restore,@function
lrs_restore:
	movl $15, %eax
	syscall
