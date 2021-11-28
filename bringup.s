.global start
.global _start
.global main
.global _main

// Apparently the entrypoint needs to be aligned to a 12 byte boundary so iBoot can find it
// Something to do with IORVBAR
.align 5
_start:
start:
main:
_main:
	/*
	* main entrypoint
	* iBoot sets x0 to the physical address of the boot args struct
	*
	* Setup a stack and then bounce
	*/
	adr fp, _stack_base
	mov sp, fp
	adr lr, wfi_forever

	// And we out baby
	// Don't forget that llvm adds underscores on macOS for whatever reason
	b _hypervisor_entry

wfi_forever:
	wfi
	b wfi_forever

_stack_top:
	.space 16384, 0
_stack_base:
	.space 64, 0
