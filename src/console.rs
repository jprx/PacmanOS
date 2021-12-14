use core::fmt;
use crate::framebuffer::SCREEN_WIDTH;
use crate::framebuffer::SCREEN_HEIGHT;
use crate::framebuffer::get_framebuffer;
use font8x8::legacy::BASIC_LEGACY;

pub const CONSOLE_WIDTH  : usize = SCREEN_WIDTH / 16;
pub const CONSOLE_HEIGHT : usize = SCREEN_HEIGHT / 16;

// See: https://os.phil-opp.com/vga-text-mode/#a-println-macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_console_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/*
 * Console
 * An object representing the current text console drawn to the screen
 */
pub struct Console {
    pub x : usize,
    pub y : usize,
}

// The global cursor position & console state
pub static mut global_console : Console = Console::new();

impl Console {
    pub const fn new() -> Console {
        return Console {
            x: 0,
            y: 0,
        }
    }

    pub fn write_char(&mut self, c: char) {
        let framebuffer = get_framebuffer();
        let screen_x_cursor_start = 8 * self.x;
        let mut screen_x_cursor = screen_x_cursor_start;
        let mut screen_y_cursor = 8 * self.y;

        if c == '\n' {
            self.newline();
            return;
        }

        for char_data in &BASIC_LEGACY[c as usize] {
            for bit in 0..8 {
                if *char_data & (1 << bit) != 0 {
                    if (((2 * screen_x_cursor) + 1) < SCREEN_WIDTH) && (((2 * screen_y_cursor) + 1) < SCREEN_HEIGHT) {
                        framebuffer[2*(screen_y_cursor as usize)][2*(screen_x_cursor as usize)] = 0xffffffff;
                        framebuffer[2*(screen_y_cursor as usize)+1][2*(screen_x_cursor as usize)] = 0xffffffff;
                        framebuffer[2*(screen_y_cursor as usize)][2*(screen_x_cursor as usize)+1] = 0xffffffff;
                        framebuffer[2*(screen_y_cursor as usize)+1][2*(screen_x_cursor as usize)+1] = 0xffffffff;
                    }
                }
                screen_x_cursor += 1;
            }
            screen_x_cursor = screen_x_cursor_start;
            screen_y_cursor += 1;
        }

        self.inc_cursor();
    }

    pub fn newline(&mut self) {
        if self.y < CONSOLE_HEIGHT - 1 {
            self.x = 0;
            self.y += 1;
        }
        else {
            // @TODO scrolling
        }
    }

    pub fn inc_cursor(&mut self) {
        if self.x < CONSOLE_WIDTH - 1{
            self.x += 1;
        }
        else {
            self.newline();
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.bytes() {
            self.write_char(c as char);
        }
    }
}

impl fmt::Write for Console {
    fn write_str (&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(())
    }
}

// Methods to support the macro
#[doc(hidden)]
pub fn _console_print (args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        global_console.write_fmt(args).unwrap();
    }
}
