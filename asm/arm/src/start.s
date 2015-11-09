.global _start
.type _start,%function
_start:
	mov fp,#0
	mov lr,#0
	ldr a1,[sp],#4
	mov a2,sp
	bl __lrs_start_main
1:	b 1b
