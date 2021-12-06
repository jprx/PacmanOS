// iBoot arguments passed to us in x0
// All pointers are physical addresses

pub const BOOT_LINE_LENGTH : usize = 608;

#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct BootVideo {
	pub baseaddr 	: 	u64, // Base address of video memory
	pub display 	: 	u64, // Display Code
	pub rowbytes 	: 	u64, // Bytes per pixel row
	pub width 		: 	u64, // Width
	pub height 		: 	u64, // Height
	pub depth 		: 	u64, // Pixel Depth and other parameters
}

#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct iBootArgs {
	pub Revision			: u16,              			// Revision of boot_args structure
	pub Version				: u16,           				// Version of boot_args structure
	pub virtBase 			: u64,          				// Virtual base of memory
	pub physBase    		: u64,          				// Physical base of memory
	pub memSize 			: usize,          				// Size of memory
	pub topOfKernelData 	: u64,        					// Highest physical address used in kernel data area
	pub Video    			: BootVideo,        			// Video Information
	pub machineType   		: u32,           				// Machine Type
	pub deviceTreeP  		: usize,       					// Base of flattened device tree
	pub deviceTreeLength 	: u32,     						// Length of flattened tree
	pub CommandLine 		: [char; BOOT_LINE_LENGTH],  	// Passed in command line
	pub bootFlags  			: u64,             				// Additional flags specified by the bootloader
	pub memSizeActual  		: u64,      					// Actual size of memory
}

#[derive(Copy,Clone,Debug)]
pub enum VirtMode {
	Baremetal,
	Qemu,
}
