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
#![feature(const_mut_refs)]

use core::panic::PanicInfo;

#[macro_use]
mod console;
mod iboot;
mod logo;
mod framebuffer;

use crate::iboot::iBootArgs;
use crate::logo::pacman_logo;
use crate::framebuffer::color10bto8b;
use font8x8::legacy::BASIC_LEGACY;
use crate::framebuffer::SCREEN_WIDTH;
use crate::framebuffer::SCREEN_HEIGHT;
use crate::console::Console;


pub static mut global_console : Console = Console::new();

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

    // for y in 0 .. 1080 {
    //     for x in 0 .. 1920 {
    //         vidmem[y as usize][x as usize] = color10bto8b(pacman_logo[y as usize][x as usize]);
    //         // vidmem[y as usize][x as usize] = 0xffffffff;
    //     }
    // }
    // let mut screen_x_cursor_start = unsafe { 8 * global_console.x };
    // let mut screen_x_cursor = screen_x_cursor_start;
    // let mut screen_y_cursor = unsafe { 8 * global_console.y };
    // let framebuffer = framebuffer::get_framebuffer();

    // for char_data in &BASIC_LEGACY['p' as usize] {
    //     for bit in 0..8 {
    //         if *char_data & (1 << bit) != 0 {
    //             if (screen_x_cursor < SCREEN_WIDTH) && (screen_y_cursor < SCREEN_HEIGHT) {
    //                 framebuffer[screen_y_cursor as usize][screen_x_cursor as usize] = 0xffffffff;
    //             }
    //         }
    //         screen_x_cursor += 1;
    //     }
    //     screen_x_cursor = screen_x_cursor_start;
    //     screen_y_cursor += 1;
    // }

    let mut osconsole = console::Console::new();
    osconsole.write_char('h');
    osconsole.write_char('i');
    osconsole.write_string("\nHello, PacmanOS!");

    print!("Hello World!");

    loop {}
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
        adrp fp, _stack_bot
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
