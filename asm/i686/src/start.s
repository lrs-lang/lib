.global _start
.text
_start:
	xor %ebp,%ebp
	pop %ecx
	mov %esp,%eax
	and $-16,%esp
	push %esp
	push %esp
	push %eax
	push %ecx
	call __lrs_start_main
1:	jmp 1b
