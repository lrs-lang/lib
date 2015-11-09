.global _start
.type _start,@function
_start:
	xor %rbp,%rbp
	mov %rdx,%r9
	pop %rsi
	mov %rsp,%rdx
	andq $-16,%rsp
	call __lrs_start_main
1:	jmp 1b
