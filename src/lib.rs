#![no_std]
#![feature(asm)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![feature(abi_x86_interrupt)]
#![allow(non_camel_case_types)]
#![feature(const_fn_fn_ptr_basics)]
#![allow(unused_parens)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(str_split_as_str)]

#![feature(asm_const)]
#![allow(named_asm_labels)]

use core::panic::PanicInfo;


#[allow(unused_imports)]

extern "C" {
    fn hypervisor_entry();
}


#[no_mangle]
pub extern "C" fn rust_start (_iboot_info: usize) {
    unsafe {
        hypervisor_entry();
    }
}

#[panic_handler]
pub extern "C" fn rust_panic (_info: &PanicInfo) -> ! {
    let mut x = 0;
    loop {
        x = x + 1;
    }
}

// Attempt to do everything including iBoot arg reading & stack initialization within Rust
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start_rust () {
    asm!{
        "adrp fp, _stack_bot
        mov sp, fp
        adrp lr, wfi_forever
        b rust_start",
        options(noreturn)
    }
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn wfi_forever () {
    asm!{
        "wfi
        b wfi_forever",
        options(noreturn)
    }
}