// Support for Qemu serial as well as the Apple M1 serial port
// This module should transparently switch based on virt.rs's current operating mode

pub const VIRT_UART0 : u64 = 0x09000000;

pub struct Serial {
	// The base address of this UART device
	pub baseaddr: u64,
}

impl Serial {
	pub const fn new() -> Self {
		// Default to the Qemu device (TODO: CHANGE THIS FOR M1 BARE METAL!)
		return Serial{
			baseaddr: VIRT_UART0
		}
	}

	pub fn write_char (&mut self, c: char) {
		let transmit_port = self.baseaddr as *mut char;
		unsafe {
			// Let's hope this base address is correct
			core::ptr::write_volatile(transmit_port, c);
		}
	}

	pub fn write_string (&mut self, s: &str) {
		for c in s.bytes() {
			self.write_char(c as char);
		}
	}
}

impl core::fmt::Write for Serial {
	fn write_str (&mut self, s: &str) -> core::fmt::Result {
		self.write_string(s);
		return Ok(())
	}
}
