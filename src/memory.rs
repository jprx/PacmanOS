// Methods for managing virtual memory
use bitfield::bitfield;

// Definitions for the ID_AA64MMFR0_EL1 MSR
// See: https://developer.arm.com/documentation/ddi0488/c/system-control/aarch64-register-descriptions/aarch64-memory-model-feature-register-0--el1
// Each field is "GETTER", "SETTER", MSB, LSB
// Where MSB is the most significant bit occupied, and LSB is the least significant bit occupied
// Leaving "SETTER" makes the field read only
bitfield! {
	pub struct IDAA64MMFR0EL1(u64);
	pub PARange, set_PARange: 3, 0;
	pub ASIDBits, _: 7, 4;
	pub BigEnd, _: 11, 8;
	pub SNSMem, _: 15, 12;
	pub res0, _: 19, 16;
	pub supports_16k, _: 23, 20;
	pub supports_64k, _: 27, 24;
	pub supports_4k, _: 31, 28;
	pub res1, _: 63, 32;
}

impl core::fmt::Debug for IDAA64MMFR0EL1 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "0x{:X}: 16K Supported: {}, 64K Supported: {}, 4K Supported: {}",
			self.0,
			if self.supports_16k() == 0 { "Yes" } else { "No" },
			if self.supports_64k() == 0 { "Yes" } else { "No" },
			if self.supports_4k() == 0 { "Yes" } else { "No" },
		)
	}
}
