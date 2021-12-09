
/* 1 GB = 1024M */

/*MEMORY {
	FLASH (RXI) : ORIGIN = 0x00000000, LENGTH = 0x08000000
	RAM : ORIGIN = 1024M, LENGTH = 1024M
}*/

ENTRY(_start);

SECTIONS {
	/*. = 0x40000000;*/
	. = 1024M;
	.text : ALIGN(16K) { *(.bringup.init); *(.text); }
	.data : ALIGN(16K) { *(.data); }
	.bss : ALIGN(16K) { *(.bss); }
	.rodata : ALIGN(16K) { *(.rodata); }
	. = . + 1M;
	.stack_alignment : ALIGN(16K) {}
	PROVIDE(_stack_bot = .);
}
