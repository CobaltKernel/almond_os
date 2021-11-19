//! Handles VGA Buffer Creation, Manipulation

use spin::Mutex;
use volatile::Volatile;


const ATTR_ADDR_DATA_REG:      u16 = 0x3C0;
const ATTR_DATA_READ_REG:      u16 = 0x3C1;
const SEQUENCER_ADDR_REG:      u16 = 0x3C4;
const DAC_ADDR_WRITE_MODE_REG: u16 = 0x3C8;
const DAC_DATA_REG:            u16 = 0x3C9;
const GRAPHICS_ADDR_REG:       u16 = 0x3CE;
const CRTC_ADDR_REG:           u16 = 0x3D4;
const CRTC_DATA_REG:           u16 = 0x3D5;
const INPUT_STATUS_REG:        u16 = 0x3DA;

const BUFFER_PTR: *mut u8 = 0xb8000 as *mut _;


lazy_static::lazy_static! {
    static ref BUFFER: Mutex<&'static mut TextBuffer> = Mutex::new(unsafe {TextBuffer::new(BUFFER_PTR)});
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
/// Utility Enum That Holds A Color. Defaults To Black
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl From<u8> for Color {
    fn from(byte: u8) -> Self {
        match byte & 0xF {
            0 => Self::Black,
            1 => Self::Blue, 
            2 => Self::Green,
            3 => Self::Cyan,
            4 => Self::Red,
            5 => Self::Magenta,
            6 => Self::Brown,
            7 => Self::LightGray,
            8 => Self::DarkGray,
            9 => Self::LightBlue, 
            10 => Self::LightGreen,
            11 => Self::LightCyan,
            12 => Self::LightRed,
            13 => Self::Pink,
            14 => Self::Yellow,
            15 => Self::White,
            _ => {Self::default()}
        }
    }
} 


/// A Single VGA Character
pub type Character = u8;

/// The VGA Color Information,
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ColorAttrib(Color, Color);

impl ColorAttrib {
    /// Get The Underlying VGA Color Attribute Code
    pub fn raw(&self) -> u8 {
        (self.0 as u8) << 4 | (self.1 as u8)
    }

    /// Construct A New Color Attribute
    pub fn new(bg: Color, fg: Color) -> Self {
        Self(bg, fg)
    }

    /// Construct An Instance From A Byte
    pub fn from(byte: u8) -> Self {
        Self(Color::from(byte >> 4), Color::from(byte))
    }

    /// The Background Color
    pub fn bg(&self) -> Color {
        self.0
    }

    /// The Foreground Color
    pub fn fg(&self) -> Color {
        self.1
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Represents A Single VGA Character
/// The Even Byte holds the codepoint, the odd byte holds the color information;
pub struct ScreenChar {
    chr: Character,
    color: u8,
}

#[derive(Debug)]
#[repr(transparent)]
/// Represents A Single 80x25 VGA Textmode Buffer
pub struct TextBuffer {
    contents: [[Volatile<ScreenChar>; 80]; 25],
}

impl TextBuffer {
    /// Creates A &'static mut Textbuffer From A u8-ptr.
    /// The Function Is Unsafe As The Caller MUST Guarantee That the Pointer Is Not in use by
    /// anything within 32-Kilobytes. 
    pub unsafe fn new(ptr: *mut u8) -> &'static mut TextBuffer {
        &mut *(ptr as *mut TextBuffer)
    }

    /// Overwrites The Character At (x, y) with the Supplied Color & Character.
    pub fn put_char(&mut self, x: usize, y: usize, chr: Character, color: ColorAttrib) {
        self.contents[y][x].write(ScreenChar { chr, color: color.raw() })
    }

    /// Returns A Copy Of The Data At (x, y) as a ([Character], [ColorAttrib]) tuple.
    pub fn get_char(&self, x: usize, y: usize) -> (Character, ColorAttrib) {
        let sc = self.contents[y][x].read();
        (sc.chr, ColorAttrib::from(sc.color))
    }


}

/// Writes Character Data Into The Global VGA Buffer
pub fn put_char(x: usize, y: usize, chr: Character, color: ColorAttrib) {
    crate::no_interrupt!({
        BUFFER.lock().put_char(x, y, chr, color)
    });
}

/// Reads Character Data From The Global VGA Buffer
pub fn get_char(x: usize, y: usize) -> (Character, ColorAttrib) {
    crate::no_interrupt!({
        BUFFER.lock().get_char(x, y)
    })
}

#[derive(Debug)]
/// A VGA Pallete
pub struct Palette {
    /// Color Data
    pub colors: [(u8, u8, u8); 16],
}

impl Palette {
    /// Returns The Default VGA 16-Color Palette
    pub fn default() -> Palette {
        Palette {
            colors: [
                (0x00, 0x00, 0x00), // Black
                (0x00, 0x00, 0x80), // Blue
                (0x00, 0x80, 0x00), // Green
                (0x00, 0x80, 0x80), // Cyan
                (0x80, 0x00, 0x00), // Red
                (0x80, 0x00, 0x80), // Magenta
                (0x80, 0x80, 0x00), // Brown (Dark Yellow)
                (0xC0, 0xC0, 0xC0), // Light Gray
                (0x80, 0x80, 0x80), // Dark Gray (Gray)
                (0x00, 0x00, 0xFF), // Light Blue
                (0x00, 0xFF, 0x00), // Light Green
                (0x00, 0xFF, 0xFF), // Light Cyan
                (0xFF, 0x00, 0x00), // Light Red
                (0xFF, 0x00, 0xFF), // Pink (Light Magenta)
                (0xFF, 0xFF, 0x00), // Yellow (Light Yellow)
                (0xFF, 0xFF, 0xFF), // White
            ]
        }
    }
}


