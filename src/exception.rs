// Definitions for all the possible exceptions we can face

use crate::console;

extern "C" {
	pub fn exception_vector_base_el2();
}

#[no_mangle]
pub unsafe extern "C" fn sync_exception_el2 () -> ! {
	let mut osconsole = console::Console::new();
	osconsole.write_string("HIT EXCEPTION OF KIND SYNC");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn irq_exception_el2 () -> ! {
	let mut osconsole = console::Console::new();
	osconsole.write_string("HIT EXCEPTION OF KIND IRQ");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn fiq_exception_el2 () -> ! {
	let mut osconsole = console::Console::new();
	osconsole.write_string("HIT EXCEPTION OF KIND FIQ");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn serror_exception_el2 () -> ! {
	let mut osconsole = console::Console::new();
	osconsole.write_string("HIT EXCEPTION OF KIND SERROR");
	loop{}
}

#[no_mangle]
pub unsafe extern "C" fn unk_exception () -> ! {
	let mut osconsole = console::Console::new();
	osconsole.write_string("UNKNOWN EXCEPTION");
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
