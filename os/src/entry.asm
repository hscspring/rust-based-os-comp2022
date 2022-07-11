    .section .text.entry
    .globl _start
_start:
    // 将SP设置为栈空间的栈顶
    la sp, boot_stack_top
    // 应用入口
    call rust_main

    .section .bss.stack
    // 栈底
    .globl boot_stack

boot_stack:
    // 预留64KB，作为栈空间
    .space 4096 * 16
    // 栈顶
    .globl boot_stack_top

boot_stack_top: