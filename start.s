.global _start
.extern wfi_forever

.section .start, "ax"

_start:
	nop
	nop
	nop
	adrp fp, _stack_bot
	mov sp, fp
	bl rust_start

wfi_forever:
	wfi
	b wfi_forever
