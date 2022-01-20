// Minimal support to enable and configure caches
use core::arch::asm;

pub unsafe extern "C" fn enable_caches() {
	// See "Bare-metal Boot Code for ARMv8-A Processors" Application Note
	asm!{
		"mrs x0, SCTLR_EL1",
		"orr x0, x0, #(1 << 2)", // C bit (data cache)
		"orr x0, x0, #(1 << 12)", // I bit (instruction cache)
		"msr SCTLR_EL1, x0",
		"dsb sy",
		"isb",
		lateout("x0") _
	}
}
