.global _start
.type _start,%function
_start:
    mov x29,#0
    mov x30,#0
    ldr x0,[sp,#0]
    add x1,sp,#8
    bl __lrs_start_main
