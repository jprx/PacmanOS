// Enough to get EL2 out of the way
// HCR_EL2.VM should be 0 to disable "stage 2" translation

use crate::write_msr;

/*
 * exit_el2
 * Disables EL2 and drops us into EL1
 * Should be called without the MMU enabled
 */
#[no_mangle]
pub unsafe fn exit_el2(el1_entry: u64) -> ! {
	// Reset SCTLR_EL1 per "Bare-metal Boot Code for ARMv8-A Processors" Application Note
	write_msr!("SCTLR_EL1", 0);

	// Clear HCR_EL2.VM and set HCR_EL2.RW = 1 to set EL1 to aarch64
	asm!{
		"mrs x0, HCR_EL2",
		"bic x0, x0, #1",
		"orr x0, x0, #(1 << 31)",
		"msr HCR_EL2, x0",
		lateout("x0") _
	}

	// Configure new PSTATE (returning to EL1h)
	// We will use SP_EL1 when we get to EL1
	write_msr!("SPSR_EL2", 0b00101);

	// ERET target:
	write_msr!("ELR_EL2", el1_entry);

	// Setup EL1 stack (just overwrite the EL2 boot stack):
	asm!{
		"adrp x0, _stack_bot",
		"msr SP_EL1, x0",
		lateout("x0") _
	}

	// Set CPACR_EL1.FPEN so that EL1 can use SIMD instructions without trapping
	asm! {
		"mrs x0, CPACR_EL1",
		"orr x0, x0, #(1 << 21)",
		"orr x0, x0, #(1 << 20)",
		"msr CPACR_EL1, x0",
		lateout("x0") _
	}

	// Peace be with you
	asm!{
		"
		mov x0, XZR
		mov x1, XZR
		mov x2, XZR
		mov x3, XZR
		mov x4, XZR
		mov x5, XZR
		mov x6, XZR
		mov x7, XZR
		mov x8, XZR
		mov x9, XZR
		mov x10, XZR
		mov x11, XZR
		mov x12, XZR
		mov x13, XZR
		mov x14, XZR
		mov x15, XZR
		mov x16, XZR
		mov x17, XZR
		mov x18, XZR
		mov x19, XZR
		mov x20, XZR
		mov x21, XZR
		mov x22, XZR
		mov x23, XZR
		mov x24, XZR
		mov x25, XZR
		mov x26, XZR
		mov x27, XZR
		mov x28, XZR
		mov x29, XZR
		mov x30, XZR

		eret
		",
		options(noreturn)
	}

	// asm!{
	// 	// Reset SCTLR_EL1 per "Bare-metal Boot Code for ARMv8-A Processors" Application Note
	// 	"msr SCTLR_EL1, XZR",

	// 	// Clear HCR_EL2.VM and set HCR_EL2.RW = 1 to set EL1 to aarch64
	// 	"mrs x0, HCR_EL2",
	// 	"bic x0, x0, #1",
	// 	"orr x0, x0, #(1 << 31)",
	// 	"msr HCR_EL2, x0",

	// 	// Configure SPSR_EL2 (this is the new PSTATE)
	// 	"mov x0, #0b00101", // DAIF = 0000
	// 	"msr SPSR_EL2, x0",

	// 	// See ya l8r
	// 	"eret",
	// 	options(noreturn)
	// }
}
