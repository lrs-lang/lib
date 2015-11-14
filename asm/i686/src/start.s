.global _start
.text
_start:
	xor %ebp,%ebp
	mov %esp,%eax
	and $-16,%esp ; GCC ABI requires 16 bytes alignment before the call instruction
	push %eax
	push %eax
	push %eax
	push %eax
	call lrs_start_main
