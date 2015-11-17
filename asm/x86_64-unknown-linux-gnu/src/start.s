.global _start
.type _start,@function
_start:
	xor %rbp,%rbp
	mov %rsp,%rdi
	call lrs_start_main
