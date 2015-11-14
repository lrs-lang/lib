.global _start
.type _start,%function
_start:
    mov x29,#0
    mov x30,#0
    mov x0,sp
    bl __lrs_start_main
