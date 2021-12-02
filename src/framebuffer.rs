pub const SCREEN_WIDTH   : usize = 1920;
pub const SCREEN_HEIGHT  : usize = 1080;

pub const FRAMEBUFFER_ADDR : usize = 0x0000000080000000;

// Get the current framebuffer
pub const fn get_framebuffer() -> &'static mut [[u32; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    return unsafe { &mut *(FRAMEBUFFER_ADDR as *mut[[u32; SCREEN_WIDTH]; SCREEN_HEIGHT]) };
}

pub fn pack_color(r: u32, g: u32, b: u32) -> u32 {
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
