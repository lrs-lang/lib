.global lrs_restore
.type lrs_restore,@function
lrs_restore:
	movl $173, %eax
	int $0x80
