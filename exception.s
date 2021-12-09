// Ok, fine, we can use a little bit of assembly...

.global exception_vector_base_el2

.section .text

.align 16
exception_vector_base_el2:
	b sync_exception_el2

.align 7
	b irq_exception_el2

.align 7
	b fiq_exception_el2

.align 7
	b serror_exception_el2

.align 7
	b sync_exception_el2

.align 7
	b irq_exception_el2

.align 7
	b fiq_exception_el2

.align 7
	b serror_exception_el2

.rept 8192
	b unk_exception
.endr
