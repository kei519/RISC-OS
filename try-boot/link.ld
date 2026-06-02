SECTIONS {
    /* read-only */
    . = 0x10000000;
    .vectors : { *(.vectors*) }
    .text : { *(.text*) }
    .rodata : { *(.rodata*) }
    .ARM.exidx : { *(.ARM.exidx*) }

    /* writable */
    . = 0x20000000;
    .data : { *(.data*) }
    .bss : { *(.bss*) }

    PROVIDE (_kernel_end = .);
}
