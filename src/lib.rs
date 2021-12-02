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

// Take a 10 bit color R10:G10:B10:2 to 8 bit A8:R8:G8:B8
// Just chop off the lowest 2 bits
pub fn color10bto8b(c: u32) -> u32 {
    let r = c >> 24 & 0x0FF;
    let g = c >> 14 & 0x0FF;
    let b = c >> 4 & 0x0FF;
    return (0xff << 24) | (r << 18) | (g << 10) | (b << 2);
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

// Main but for Qemu
#[no_mangle]
pub unsafe extern "C" fn kmain_virt() {
    let mut vidmem : &mut [[u32; 1920]; 1080] = &mut *(0x0000000080000000 as *mut[[u32; 1920]; 1080]);
    // // vidmem[0][0] = 0xffffff;

    // let vaddr = &mut *(0x0000000080000000 as *mut u64);
    // *vaddr = 0x1234;

    for y in 0 .. 1080 {
        for x in 0 .. 1920 {
            vidmem[y as usize][x as usize] = color10bto8b(pacman_logo[y as usize][x as usize]);
            // vidmem[y as usize][x as usize] = 0xffffffff;
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
pub unsafe extern "C" fn _start () {
    // If x0 is 0 then we're in Qemu so do the _start_virt initialization instead
    // Otherwise we're on an iBoot compliant thing so probably M1 bare metal
    asm!{
        "
        cmp x0, #0
        beq _start_virt

        adrp fp, _stack_bot
        mov sp, fp
        adrp lr, wfi_forever
        b kmain",
        options(noreturn)
    }
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start_virt () {
    // Stack for virt platform should be 1 GiB (0x40000000) which is where Qemu's RAM begins
    // Qemu won't let you write to ROM / where it loads us like M1 will :(
    // I use adrp to load the new stack address- this will offset it by some positive amount which is cool
    // because stacks grow downwards
    asm!{
        "
        adrp fp, 0x40010000
        mov sp, fp
        adrp lr, wfi_forever
        b kmain_virt
        ",
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
