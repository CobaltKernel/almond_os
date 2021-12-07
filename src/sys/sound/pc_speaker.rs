//! API For Iteracting With The Internal PC Speaker.

use bit_field::BitField;
use x86_64::instructions::port::Port;

/// Set The PC Speaker's Pitch To The Given Frequency.
pub fn set_pitch(freq: f64) {
    crate::sys::timer::set_frequency_ch2(freq);
}

/// Play The Set Pitch
pub fn play() {
    let tmp: u8 = unsafe { Port::new(0x61).read() };
    if !tmp.get_bit(0) {
        let mut port: Port<u8> = Port::new(0x61);
        unsafe { port.write(tmp | 1); }
    }
}

/// Stop Playing Sound
pub fn stop() {
    let tmp: u8 = unsafe { Port::new(0x61).read() };
    if tmp.get_bit(0) {
        let mut port: Port<u8> = Port::new(0x61);
        unsafe { port.write(tmp & 0xFC); }
    }
}
