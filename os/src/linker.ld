/* 目标平台 */
OUTPUT_ARCH(riscv)
/* 入口 */
ENTRY(_start)
/* RustSBI期望的OS起始地址 */
BASE_ADDRESS = 0x80200000;


SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    /* .text代码段 */
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    etext = .;


    srodata = .;
    /* 只读数据段 */
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    erodata = .;


    sdata = .;
    /* 数据段 */
    .data : {
        *(.data .data.*)
    }
    . = ALIGN(4K);
    edata = .;


    /* bss段 */
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}