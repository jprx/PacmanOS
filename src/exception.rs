// Definitions for all the possible exceptions we can face

use crate::serial;
use crate::println;
use crate::get_el;
use core::arch::asm;

extern "C" {
	pub fn exception_vector_base_el2();
}

#[no_mangle]
pub unsafe extern "C" fn sync_exception () -> ! {
	let mut osconsole = serial::Serial::new();
	osconsole.write_string("HIT EXCEPTION OF KIND SYNC\n");

	let current_el = get_el();
	if current_el == 2 {
		osconsole.write_string("Taking exception in EL2\n");
	}

	if current_el == 1 {
		osconsole.write_string("Taking exception in EL1\n");
	}

	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn irq_exception () -> ! {
	let mut osconsole = serial::Serial::new();
	osconsole.write_string("HIT EXCEPTION OF KIND IRQ\n");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn fiq_exception () -> ! {
	let mut osconsole = serial::Serial::new();
	osconsole.write_string("HIT EXCEPTION OF KIND FIQ\n");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn serror_exception () -> ! {
	let mut osconsole = serial::Serial::new();
	osconsole.write_string("HIT EXCEPTION OF KIND SERROR\n");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn unk_exception () -> ! {
	let mut osconsole = serial::Serial::new();
	osconsole.write_string("UNKNOWN EXCEPTION\n");
	loop{}
}

pub unsafe fn set_vbar_el2 (new_baseaddr: u64) {
	asm!{
		"msr vbar_el2, {}",
		in(reg) new_baseaddr
	}
}

pub fn get_vbar_el2 () -> u64 {
	let baseaddr : u64;
	unsafe {
		asm!{
			"mrs {}, vbar_el2",
			out(reg) baseaddr
		}
	}
	return baseaddr;
}

pub unsafe fn set_vbar_el1 (new_baseaddr: u64) {
	asm!{
		"msr vbar_el1, {}",
		in(reg) new_baseaddr
	}
}

pub fn get_vbar_el1 () -> u64 {
	let baseaddr : u64;
	unsafe {
		asm!{
			"mrs {}, vbar_el1",
			out(reg) baseaddr
		}
	}
	return baseaddr;
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn exception_vector_rust () -> ! {
	asm!{
		"
.align 16
	b sync_exception

.align 7
	b irq_exception

.align 7
	b fiq_exception

.align 7
	b serror_exception

.align 7
	b sync_exception

.align 7
	b irq_exception

.align 7
	b fiq_exception

.align 7
	b serror_exception

.rept 8192
	b unk_exception
.endr
		",
		options(noreturn)
	}
}
