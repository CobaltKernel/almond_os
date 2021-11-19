//! Provides Functions For Communicating With The Programmable Interrupt Timer
//! & General Sleep Functions

use bit_field::BitField;
use x86_64::instructions::port::Port;

use crate::KResult;

use super::interrupt::idt::set_irq_handler;

/// The Main PIT Freqency, Runs At 1.983282 Mhz.
pub const PIT_FREQUENCY: f64 = 3_579_545.0 / 3.0; // 1_193_181.666 Hz
const PIT_COMMAND: u16 = 0x43;
const CH_0: u16 = 0x40;
const CH_1: u16 = 0x41;
const CH_2: u16 = 0x42;


/// Sets The Given PIT Channel's Frequency, Matching As Close As Possible.
pub fn set_frequency(channel: usize, freq: f64) {
    let mut data_port: Port<u8> = match channel & 0b11 {
        0 => Port::new(CH_0),
        1 => Port::new(CH_1),
        2 => Port::new(CH_2),
        _ => Port::new(CH_0),
    };

    let mut command_port: Port<u8> = Port::new(PIT_COMMAND);

    let mut command: u8 = 0;

    command.set_bits(6..8, (channel & 0b11) as u8);
    command.set_bits(4..6, 0b11);
    command.set_bits(1..4,  3);
    command.set_bit(0, false);

    let reload_value = (PIT_FREQUENCY / freq) as u16; 

    unsafe {
        command_port.write(command);
        data_port.write(((reload_value | 0x00FF) >> 0) as u8);
        data_port.write(((reload_value | 0xFF00) >> 8) as u8);
    }



}

/// Setup The PIT, Set the frequency To 100hz,
/// Set The IRQ0 Handler.
pub fn initialize() -> KResult<()> {
    set_frequency(0, 100.0);
    set_irq_handler(0, on_timer_tick);
    Ok(())
}

#[doc(hidden)]
pub fn on_timer_tick() {

}