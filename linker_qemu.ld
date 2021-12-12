ENTRY(_start);

MACH_MAGIC = 0xFEEDFACF;

LC_UNIXTHREAD = 0x5;
LC_SEGMENT_64 = 0x19;

/* Kernel load address */
/* The ELF is loaded at 1 GB to work with the Qemu virt machine */
/* The ELF addresses are thrown out when we objcopy, and only the Mach load commnads matter */
/* Need to shift every virtual address by this amount when creating the mach header & load commands */
KERNEL_BASE = 0xfffffe0007004000;

PROT_READ = 0x01;
PROT_WRITE = 0x02;
PROT_EXECUTE = 0x04;

SECTIONS {
	/*. = 0x40000000;*/
	/* LONG is a 32 bit word inserted directly into the section in question */

	. = 1024M;
	_loadable_begin = .;
	.macho_header : ALIGN(16K) {
		/* See: https://opensource.apple.com/source/xnu/xnu-2050.18.24/EXTERNAL_HEADERS/mach-o/loader.h */

		LONG(MACH_MAGIC);
		LONG(16777228); /* cputype from otool -h */
		LONG(2); /* cpusubtype */
		LONG(0x0c); /* filetype = 0x0C (undefined, this is what m1n1 does) */
		LONG(2); /* ncmds */
		LONG(_COMMANDS_END - _COMMANDS_START); /* sizeofcmds */
		LONG(4); /* flags */
		LONG(0); /* reserved */

		/* Just load the entire kernel image into one massive section */
		_COMMANDS_START = .;

		/* UNIXTHREADS is just used to specify PC */
		LONG(LC_UNIXTHREAD); /* Command kind */
		LONG(0x120); /* Size of this command */
		LONG(6); /* ARM_THREAD64 */
		LONG(0x44); /* length */
		. += 32 * 8; /* useless registers */
		QUAD(_start + KERNEL_BASE) /* pc */
		. += 8; /* useless registers */

		/* SEGMENT_64 that encompasses the whole binary */
		LONG(LC_SEGMENT_64); /* Command kind */
		LONG(0x48); /* Size of this command */
		LONG(0x004C4C41); /* Segment name = "ALL" */
		. = . + 12;
		QUAD(KERNEL_BASE); /* Memory address of this segment */
		QUAD(_loadable_end - _loadable_begin); /* Size of the segment to load */
		QUAD(0); /* File offset of this segment */
		QUAD(_loadable_end - _loadable_begin); /* File size to load */
		LONG(PROT_READ | PROT_WRITE | PROT_EXECUTE); /* maxprot */
		LONG(PROT_READ | PROT_WRITE | PROT_EXECUTE); /* initprot */
		LONG(0); /* nsects */
		LONG(0); /* flags */

		_COMMANDS_END = .;
	}
	.text : ALIGN(16K) { *(.bringup.init); *(.text); }
	.rodata : ALIGN(16K) { *(.rodata); }
	.bss : ALIGN(16K) { *(.bss); }
	.data : ALIGN(16K) {
		*(.data);

		. = . + 1M;
		_stack_bot = .;
	}

	_loadable_end = .;
}
