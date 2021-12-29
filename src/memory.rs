// Methods for managing virtual memory
// We use 16 KB granule

/*

TLDR on regs:

HCR_EL2.VM should be 0 to disable "stage 2" translation

*/
use bitfield::bitfield;
use bitflags::bitflags;
use crate::read_msr;
use crate::write_msr;

pub const NUM_TABLE_ENTRIES : usize = 2048;

// Points to another table
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct TableEntry(u64);

// Points to a target page (either huge page or end of walk)
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct BlockEntry(u64);

// 16 KB aligned page table
// Store every entry as a u64, we are responsible for managing which entry is what kind
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
#[repr(C, align(16384))]
pub struct PageTable {
	entries: [u64; NUM_TABLE_ENTRIES]
}

// SCTLR_EL1.M is bit 0
pub const SCTLR_EL1_FLAG_ENABLE_MMU : u64 = 0b01;

/*
 Meaningful TCR fields:
 Derived from 0x3520 used in TCR_EL3 by "Bare-metal Boot Code for ARMv8-A Processors" Application Note
 And from osfmk/arm64/proc_reg.h in XNU

 TBID1[52]		= 0 (TCR_EL1.TBI1 applies to Instruction and Data accesses.) This is what XNU does.
 TBID0[51] 		= 1 (TCR_EL1.TBI0 applies to Data accesses only.) This is what XNU does.
 TBI1[38]	 	= 0 (Top Byte used in the address calculation.) This is what XNU does.
 TBI0[37] 		= 1 (Top Byte ignored in the address calculation.) This is what XNU does.
 IPS[34:32] 	= 0b011 (42 bits, 4TB) what XNU seems to use by static analysis (that or 40 bits)
 TG1[31:30] 	= 0b01 (16KB)
 SH1[29:28] 	= 0b11 (Inner Shareable.)
 ORGN1[27:26] 	= 0b01 (Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable.)
 IRGN1[25:24] 	= 0b01 (Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable.)
 T1SZ[21:16] 	= 16 (2^48 bytes in upper address space)
 TG0[15:14] 	= 0b10 (16KB). Note that this is a different encoding than TG1!!!
 SH0[13:12] 	= 0b11 (Inner Shareable)
 ORGN0[11:10] 	= 0b01 (Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable.)
 IRGN0[9:8] 	= 0b01 (Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable.)
 T0SZ[5:0] 		= 16 (2^48 bytes in lower address space)
 */
pub const TCR_EL1_VALUE : u64 = 0x800237510B510; // 0x37520B520; // 0x800237520B520;

// For TTBR0
pub static mut GLOBAL_L0_TABLE0 : PageTable = PageTable {
	entries: [0; NUM_TABLE_ENTRIES]
};

pub static mut GLOBAL_L1_TABLE : PageTable = PageTable {
	entries: [0; NUM_TABLE_ENTRIES]
};

pub static mut GLOBAL_L2_TABLE : PageTable = PageTable {
	entries: [0; NUM_TABLE_ENTRIES]
};

// For TTBR1
pub static mut GLOBAL_L0_TABLE1 : PageTable = PageTable {
	entries: [0; NUM_TABLE_ENTRIES]
};

/*
 * init
 * Configures the MSRs to set up paging.
 *
 * Call this from EL1 please.
 * Should drop us into an identity mapped EL1 execution context.
 */
pub unsafe fn init() {

	GLOBAL_L0_TABLE0.entries[0] = TableEntry::new(
								(&mut GLOBAL_L1_TABLE as *mut _) as u64,
								TableEntryFlags::Valid | TableEntryFlags::Kind,
							).to_u64();

	GLOBAL_L1_TABLE.entries[0] = BlockEntry::new(
								0 as u64,
								BlockEntryFlags::Valid | BlockEntryFlags::AF,
							).to_u64();

	GLOBAL_L1_TABLE.entries[1] = TableEntry::new(
								(&mut GLOBAL_L2_TABLE as *mut _) as u64,
								TableEntryFlags::Valid | TableEntryFlags::Kind,
							).to_u64();

	for i in 0..NUM_TABLE_ENTRIES {
		GLOBAL_L2_TABLE.entries[i] = BlockEntry::new(
								(i << 25) as u64,
								BlockEntryFlags::Valid | BlockEntryFlags::AF,
							).to_u64();
	}

	write_msr!("TCR_EL1", TCR_EL1_VALUE);
	write_msr!("TTBR0_EL1", (&GLOBAL_L0_TABLE0 as *const _) as u64);
	write_msr!("TTBR1_EL1", (&GLOBAL_L0_TABLE0 as *const _) as u64);
	mmu_enable();
}

pub unsafe fn mmu_enable() {
	// let sctlr_el1 = read_msr!("SCTLR_EL1");
	// write_msr!("SCTLR_EL1", sctlr_el1 | SCTLR_EL1_FLAG_ENABLE_MMU);

	asm!{
		"mrs x0, SCTLR_EL1",
		"orr x0, x0, #0x1",
		"msr SCTLR_EL1, x0",
		"dsb sy",
		"isb",
		lateout("x0") _
	}
}

pub unsafe fn mmu_disable () {
	let sctlr_el1 = read_msr!("SCTLR_EL1");
	write_msr!("SCTLR_EL1", sctlr_el1 & !SCTLR_EL1_FLAG_ENABLE_MMU);
}

/*
Table Entries and Block Entries have different bits. :(

Table entries point to further tables (or are the last table in the translation regime).
Block entries are basically just huge pages (point at a block). Or last level in table translation.

Table descriptors only have upper attributes.
Block/ page descriptors have upper and lower attributes.

In general, all entries (table entry, block / page entry) look like this:
+-----------------------------------------------------------------------------------------+
| Upper Attributes | Reserved | Next Address | Reserved | Lower Attributes | Size | Valid |
+-----------------------------------------------------------------------------------------+

Except table entries don't have lower attributes

*/

bitflags! {
	pub struct TableEntryFlags : u64 {
		const Valid = (1 << 0);

		// 0 = points to another table
		// 1 = points to a huge page
		// This should always be 1 for Table Entries
		const Kind = (1 << 1);

		// Non-secure bit (res0 for us since we aren't using secure mode)
		const NS = (1 << 63);

		// AP[1:0] is bits [62, 61]. AP[1] is basically "Read only" to all levels if set to 1
		// AP[0] is basically "can EL0 access" (yes when set to 1, no when set to 0)
		// See D5.4.5 Data access permission controls in ARM specification on page D5-2737.
		const ReadOnly = (1 << 62);
		const UnprivilegedAccess = (1 << 61);

		// Unprivileged execute never
		const UXN = (1 << 60);

		// Privileged execute never
		const PXN = (1 << 59);
	}
}

bitflags! {
	pub struct BlockEntryFlags : u64 {
		const Valid = (1 << 0);

		// This should be 0 for Block Entries, 1 for Page Entries (level 3)
		// (Page Entry = last level of translation, level 3 for us)
		// This makes it effectively !HugePage so I call it SmallPage here
		const SmallPage = (1 << 1);

		// Unprivileged execute never
		const UXN = (1 << 54);

		// Privileged execute never
		const PXN = (1 << 53);

		const Contiguous = (1 << 52);

		// Dirty bit modifier
		const DBM = (1 << 51);

		// Guarded page
		const GP = (1 << 50);

		// Block translation entry
		// TLDR if this is 1 then this block entry will never go into TLB
		const nT = (1 << 16);

		// Bits [15:12] define OA[51:50] if using 52 bit address. We can ignore this

		// Not global bit
		const nG = (1 << 11);

		// Access flag
		// As this is managed in software, accessing a block entry with AF != 1
		// will result in a fault!
		const AF = (1 << 10);

		// Shareability field, ignored for now
		const SH1 = (1 << 9);
		const SH0 = (1 << 8);

		// Access policy / data access permission controls bits
		// See Section D5.4.5 on page D5-2735
		const ReadOnly = (1 << 7);
		const UnprivilegedAccess = (1 << 6);

		const NS = (1 << 5);

		// AttrIndex is bits [4:2], ignored for now
	}
}

impl TableEntry {
	pub const fn invalid() -> Self {
		return TableEntry(0)
	}

	#[inline]
	pub fn get_flags(&self) -> TableEntryFlags {
		// from_bits_truncate will ignore any fields that aren't defined in the struct
		return TableEntryFlags::from_bits_truncate(self.0);
	}

	#[inline]
	pub fn set_flags(&mut self, new_flags: TableEntryFlags) -> &mut Self {
		self.0 = self.address() | new_flags.bits();
		return self;
	}

	// Get the address of the table we point to (bits [47:14])
	#[inline]
	pub fn address(&self) -> u64 {
		return self.0 & 0x0000_FFFFFFFFC_000;
	}

	// Set the address of the table we point to without modifying flags
	#[inline]
	pub fn set_address(&mut self, new_addr: u64) -> &mut Self {
		let flags = self.get_flags();
		self.0 = new_addr as u64 | flags.bits();
		return self;
	}

	pub fn is_valid(&self) -> bool {
		return self.get_flags().contains(TableEntryFlags::Valid);
	}

	pub fn new(base_addr: u64, flags: TableEntryFlags) -> Self {
		let mut new_entry = TableEntry::invalid();
		new_entry.set_address(base_addr);
		new_entry.set_flags(flags);
		return new_entry;
	}

	pub fn to_u64(&self) -> u64 {
		return self.0;
	}

	pub fn from_u64(in_u64: u64) -> Self {
		return TableEntry(in_u64);
	}
}

impl BlockEntry {
	pub const fn invalid() -> Self {
		return BlockEntry(0)
	}

	#[inline]
	pub fn get_flags(&self) -> BlockEntryFlags {
		// from_bits_truncate will ignore any fields that aren't defined in the struct
		return BlockEntryFlags::from_bits_truncate(self.0);
	}

	#[inline]
	pub fn set_flags(&mut self, new_flags: BlockEntryFlags) -> &mut Self {
		self.0 = self.address() | new_flags.bits();
		return self;
	}

	// Get the address of the block we point to (bits [47:14])
	// For level 2 pages the bits are actually bits [47:25] but that's ok
	#[inline]
	pub fn address(&self) -> u64 {
		return self.0 & 0x0000_FFFFFFFFC_000;
	}

	// Set the address of the block we point to without modifying flags
	#[inline]
	pub fn set_address(&mut self, new_addr: u64) -> &mut Self {
		let flags = self.get_flags();
		self.0 = new_addr as u64 | flags.bits();
		return self;
	}

	pub fn is_valid(&self) -> bool {
		return self.get_flags().contains(BlockEntryFlags::Valid);
	}

	pub fn new(base_addr: u64, flags: BlockEntryFlags) -> Self {
		let mut new_entry = BlockEntry::invalid();
		new_entry.set_address(base_addr);
		new_entry.set_flags(flags);
		return new_entry;
	}

	pub fn to_u64(&self) -> u64 {
		return self.0;
	}

	pub fn from_u64(in_u64: u64) -> Self {
		return BlockEntry(in_u64);
	}
}

// Definitions for the ID_AA64MMFR0_EL1 MSR
// See: https://developer.arm.com/documentation/ddi0488/c/system-control/aarch64-register-descriptions/aarch64-memory-model-feature-register-0--el1
// Each field is "GETTER", "SETTER", MSB, LSB
// Where MSB is the most significant bit occupied, and LSB is the least significant bit occupied
// Leaving "SETTER" makes the field read only
// Adding "impl Debug" on the second line lets you get an automatic Debug implementation
bitfield! {
	pub struct IDAA64MMFR0EL1(u64);
	// impl Debug;
	pub PARange, _: 3, 0;
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
