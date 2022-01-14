//! Keyboard Input Functions
use alloc::string::String;
use lazy_static::lazy_static;
use pc_keyboard::{layouts::Uk105Key, DecodedKey, HandleControl::Ignore, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

type KeyboardUk = Keyboard<Uk105Key, ScancodeSet1>;

use crate::{KResult, print};

use super::{mem::ringbuffer::RingBuffer256};

/// ASCII DELETE KEY (0x7F)
pub const DELETE: char = '\x7f';

/// ASCII BACKSPACE KEY (0x08)
pub const BACKSPACE: char = '\x08';

/// ASCII NEW_LINE KEY (0x08)
pub const NEW_LINE: char = '\n';

lazy_static! {
    static ref KEYBOARD_BUFFER: Mutex<Option<RingBuffer256<u8>>> = Mutex::new(None);
    static ref KEYBOARD: Mutex<KeyboardUk> =
        Mutex::new(KeyboardUk::new(Uk105Key, ScancodeSet1, Ignore));
}

static mut LAST_KEY: char = 0 as char;

/// Initialize The Input System
pub fn initialize() -> KResult<()> {
    (*KEYBOARD_BUFFER.lock()) = Some(RingBuffer256::new());

    crate::sys::interrupt::set_irq_handler(1, on_key_pressed);

    Ok(())
}

/// IRQ1 handler
pub fn on_key_pressed(_: u8) {
    let byte = unsafe { Port::<u8>::new(0x60).read() };
    let mut kb = KEYBOARD.lock();
    if let Ok(Some(event)) = kb.add_byte(byte) {
        if let Some(key) = kb.process_keyevent(event) {
            match key {
                DecodedKey::RawKey(_) => { /*terminal::process_raw_key(kc)*/ }
                DecodedKey::Unicode(codepoint) => unsafe {
                    LAST_KEY = codepoint;
                },
            }
        }
    }
}

/// Reads A Single Character From The Keyboard, Blocks If No Key Is Available.
pub fn read_key() -> Option<char> {
    unsafe {
        let kc = LAST_KEY;
        LAST_KEY = '\x00';
        if kc == '\x00' {
            return None;
        } else {
            return Some(kc);
        }
    }
}


/// Read Input From The User
pub fn input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}\r", prompt);
    'input_loop: loop {
        if let Some(key) = read_key() {

            match key {
                NEW_LINE => break 'input_loop,
                BACKSPACE | DELETE => {s.pop();},
                _ => s.push(key),
            }
            print!("{}{}{}\r", prompt, s, " ".repeat(1));
        }
    }
    print!("\n");
    s
}