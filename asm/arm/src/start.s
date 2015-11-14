.global _start
.type _start,%function
_start:
	mov fp,#0
	mov lr,#0
	mov a1,sp
	bl __lrs_start_main
