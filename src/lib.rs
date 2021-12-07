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
mod virt;

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
pub unsafe extern "C" fn kmain (iboot_info: *mut iBootArgs) -> ! {
    virt::CurrentMode = virt::VirtMode::Baremetal;
    framebuffer::FramebufferAddress = (*iboot_info).Video.baseaddr;

    let vidmem = framebuffer::get_framebuffer();
    for y in 0 .. 1080 {
        for x in 0 .. 1920 {
            vidmem[y as usize][x as usize] = pacman_logo[y as usize][x as usize];
        }
    }

    let mut osconsole = console::Console::new();
    osconsole.write_char('\n');
    osconsole.write_char('\n');
    osconsole.write_char('\n');
    osconsole.write_char('\n');
    osconsole.write_char('\n');
    osconsole.write_char('\n');
    osconsole.write_char('H');
    osconsole.write_char('i');
    osconsole.write_string("\nHello PacmanOS\n");

    let current_el = get_el();

    if current_el == 0 {
        osconsole.write_string("Currently in EL0\n");
    }
    if current_el == 1 {
        osconsole.write_string("Currently in EL1\n");
    }
    if current_el == 2 {
        osconsole.write_string("Currently in EL2\n");
    }

    println!("Booting PacmanOS in {:?} mode at EL{}", unsafe { virt::CurrentMode }, get_el());

    osconsole.write_string("Just finished trying to call println!\n");

    loop {}

    common_main();
}

// Main but for Qemu
#[no_mangle]
pub unsafe extern "C" fn kmain_virt() -> ! {
    let vidmem = framebuffer::get_framebuffer();
    for y in 0 .. 1080 {
        for x in 0 .. 1920 {
            vidmem[y as usize][x as usize] = color10bto8b(pacman_logo[y as usize][x as usize]);
        }
    }

    // The instant we write to any static data living in the flash, Qemu slows down a TON
    // This is a consequence of the current hack to make flash writeable. Really need to
    // implement proper Mach-O loading.
    virt::CurrentMode = virt::VirtMode::Qemu;

    // @TODO: Fix logo colors for ramfb

    common_main();
}

// Main method called by both bare metal and qemu modes
// Any virtualization hacks should be transparent at this point
pub unsafe extern "C" fn common_main() -> ! {
    let vidmem = framebuffer::get_framebuffer();

    println!("");
    println!("Booting PacmanOS in {:?} mode at EL{}", unsafe { virt::CurrentMode }, get_el());

    for y in 0 .. 1080 {
        for x in 0 .. 1920 {
            vidmem[y as usize][x as usize] = pacman_logo[y as usize][x as usize];
        }
    }

    loop {}
}

#[panic_handler]
pub extern "C" fn rust_panic (_info: &PanicInfo) -> ! {
    let mut x = 0;
    loop {
        x = x + 1;
    }
}

pub extern "C" fn get_el() -> u64 {
    let current_el : u64;
    unsafe {
        asm!{
            "mrs {}, CurrentEL",
            out(reg) current_el
        }
    }
    return (current_el >> 2) & 0x03;
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
