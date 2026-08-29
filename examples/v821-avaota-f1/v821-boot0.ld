OUTPUT_ARCH(riscv)
ENTRY(_boot)

MEMORY
{
    /* BootROM copies the SPI NOR Boot0 image into V821 SRAM C. */
    /* Leave 0x02020000..0x02021bff to the BootROM FEL work area. */
    SRAM (rwx) : ORIGIN = 0x02000000, LENGTH = 0x00020000
}

SECTIONS
{
    .head : ALIGN(4)
    {
        KEEP(*(.head))
    } > SRAM

    .text : ALIGN(4)
    {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > SRAM

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > SRAM

    .data : ALIGN(4)
    {
        __global_pointer$ = . + 0x800;
        *(.data .data.*)
        *(.sdata .sdata.*)
        __image_end = .;
    } > SRAM

    .bss (NOLOAD) : ALIGN(4)
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        __bss_end = ALIGN(4);
    } > SRAM

    __stack_top = ORIGIN(SRAM) + LENGTH(SRAM);

    ASSERT(ADDR(.head) == ORIGIN(SRAM), "V821 eGON header is not at SRAM base")
    ASSERT(SIZEOF(.head) == 0x60, "V821 eGON header must be exactly 0x60 bytes")
    /* Keep an 8 KiB language stack clear of statics and loaded bytes. */
    ASSERT(__bss_end <= __stack_top - 0x2000, "V821 payload leaves less than 8 KiB for stack")
    /* rfel rounds up to 16 KiB; 0x18000 is the largest rounded size that
       remains below the ROM's 0x19c00 limit. */
    ASSERT(__image_end - ORIGIN(SRAM) <= 0x00018000, "V821 payload exceeds packable BootROM image limit")

    /DISCARD/ :
    {
        *(.eh_frame .eh_frame.*)
        *(.comment)
    }
}
