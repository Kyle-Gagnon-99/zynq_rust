/**
 * For the Zynq-7000, the inital memory address for the DDR is at 0x00100000
 * and the length is 512M. It happens after the OCM.
 */
MEMORY
{
    ram : ORIGIN = 0x00100000, LENGTH = 512M
}

/** ENTRY(_start) */

SECTIONS
{
    .text : {
        KEEP(*(.text._start))
        *(.text*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram

    .bss : {
        *(.bss*)
    } > ram

    .data : {
        *(.data*)
    } > ram

    /* Add the stack section */
    /* Adds the stack after the data */
    .stack (NOLOAD) : ALIGN(8) {
        _stack_start = .;
        . = ORIGIN(ram) + LENGTH(ram) - 0x4000;
        _stack_end = .;
    } > ram

    /* Discard sections that are not needed */
    /DISCARD/ : {
        *(.note.gnu.build-id)
        *(.comment)
        *(.note*)
    }
}