ENTRY(_start)

OFFSET = 0xffffffff80000000;
KERNEL_START = 0x100000;

SECTIONS
{
    . = OFFSET + KERNEL_START;

	.text : AT(ADDR(.text) - OFFSET)
	{
		*(.multiboot)
		*(.text*)
		. = ALIGN(4096);
	}

	.rodata : AT(ADDR(.rodata) - OFFSET)
	{
		*(.rodata*)
		. = ALIGN(4096);
	}

	/* Read-write data (initialized) */
	.data : AT(ADDR(.data) - OFFSET)
	{
		*(.data*)
		. = ALIGN(4096);
	}

	.bss : AT(ADDR(.bss) - OFFSET)
	{
		*(.bss)
		*(COMMON)
		. = ALIGN(4096);
	}

	kernel_image_end = .;
	low_kernel_image_end = . - OFFSET;
}