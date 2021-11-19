//! Provides More-User Friendly Terminal Functions like 'print' & 'clear';
use core::fmt::Write;

use crate::no_interrupt;

use super::vga::{self, ColorAttrib, put_char, Color};
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref WRITER: Mutex<TerminalWriter> = Mutex::new(TerminalWriter::new());
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    no_interrupt!({
        WRITER.lock().write_fmt(args).expect("Failed To Write To VGA");
    });
}

#[doc(hidden)]
pub fn _clear(fg: Color, bg: Color)  {
    no_interrupt!({
        WRITER.lock().clear_screen(ColorAttrib::new(fg, bg));
    });
}

#[doc(hidden)]
pub fn _eprint(args: core::fmt::Arguments) {
    no_interrupt!({
        let mut writer = WRITER.lock();
        let fg = writer.fg();
        let bg = writer.bg();
        writer.set_bg(Color::Red);
        writer.write_fmt(args).expect("Failed To Write To VGA");
        writer.set_bg(bg);
        writer.set_fg(fg);

    });
}

/// Handles Writing To A VGA Screen Buffer.
struct TerminalWriter {
    x: usize, 
    y: usize,

    fg_color: Color,
    bg_color: Color,
}

impl TerminalWriter {
    pub fn new() -> TerminalWriter {
        Self {
            bg_color: Color::Black,
            fg_color: Color::White,
            
            x: 0,
            y: 0,
        }
    }

    pub fn write_byte(&mut self, chr: u8) {
        if chr == b'\n' {self.newline(); return;}
        if chr == b'\r' {self.c_return(); return;}
        put_char(self.x, self.y, chr, ColorAttrib::new(self.bg_color, self.fg_color));

        self.x += 1;
        if self.x >= 80 {
            self.x = 0;
            self.newline();
        }

    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    

    fn newline(&mut self) {
        if self.y < 24 {
            self.y += 1;
        } else {
            self.shift_up();
            self.clear_row(24, ColorAttrib::new(self.bg_color, self.fg_color));
            self.y = 24;
        }
        self.c_return();
    }

    fn c_return(&mut self) {
        self.x = 0;
    }

    fn clear_row(&mut self, y: usize, color: ColorAttrib) {
        for x in 0..80 {
            put_char(x, y, b' ', color);
        }
    }

    fn clear_screen(&mut self, color: ColorAttrib) {
        for y in 0..25 {
            self.clear_row(y, color);
        }
    }

    fn shift_up(&mut self) {
        for y in 1..25 {
            for x in 0..80 {
                let (chr, color) = vga::get_char(x, y);
                vga::put_char(x, y - 1, chr, color);
            }
        }
    }

    /// Sets Bits 0:3 Of the Color Attribute. 
    pub fn set_fg(&mut self, color: Color) {
        self.fg_color = color;
    }

    /// Sets Bits 4:7 Of the Color Attribute. 
    pub fn set_bg(&mut self, color: Color) {
        self.bg_color = color;
    }

    /// Returns Bits 0:3 Of the Color Attribute. 
    pub fn fg(&self) -> Color {
        self.fg_color
    }

    /// Returns Bits 4:7 Of the Color Attribute. 
    pub fn bg(&self) -> Color {
        self.bg_color
    }
}

impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
/// Prints To The Terminal, Defaults To The VGA Terminal
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::sys::terminal::_print(format_args!($($arg)*));
    };
}

#[macro_export]
/// Prints To The Terminal, Defaults To The VGA Terminal
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::sys::terminal::_eprint(format_args!($($arg)*));
    };
}


#[macro_export]
/// Clears The Screen
macro_rules! clear {
    () => {
        {
            use $crate::sys::vga::Color;
            $crate::sys::terminal::_clear(Color::Black, Color::Yellow);
        }
    };
}
