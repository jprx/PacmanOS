
#[macro_export]
macro_rules! read_msr {
	($name:literal) => {
		{
			let __msr_val_out : u64;
			asm!{
				concat!("mrs {}, ", $name),
				out(reg) __msr_val_out
			}
			__msr_val_out
		}
	}
}

#[macro_export]
macro_rules! write_msr {
	($name:literal, $val:expr) => {
		{
			let __msr_val_in : u64 = $val;
			asm!{
				concat!("msr ", $name, ", {}"),
				in(reg) __msr_val_in
			}
		}
	}
}
