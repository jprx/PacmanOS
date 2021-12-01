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
#![allow(unused_imports)]
#![feature(asm_const)]
#![allow(named_asm_labels)]

use core::panic::PanicInfo;

#[macro_use]
mod iboot;
mod logo;

use crate::iboot::iBootArgs;
use crate::logo::pacman_logo;

pub unsafe fn pack_color(r: u32, g: u32, b: u32) -> u32 {
    return (r << 22) |
            (g << 12) |
            (b << 2);
}

// The screen better be 1920 by 1080!
#[no_mangle]
pub unsafe extern "C" fn kmain (iboot_info: *mut iBootArgs) {
    let mut vidmem : &mut [[u32; 1920]; 1080] = &mut *((*iboot_info).Video.baseaddr as *mut[[u32; 1920]; 1080]);
    for x in 0 .. (*iboot_info).Video.width {
        for y in 0 .. (*iboot_info).Video.height {
            vidmem[y as usize][x as usize] = pacman_logo[y as usize][x as usize];
        }
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
// This is the MACH-O kernel entrypoint:
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start_rust () {
    asm!{
        "adrp fp, _stack_bot
        mov sp, fp
        adrp lr, wfi_forever
        b kmain",
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
