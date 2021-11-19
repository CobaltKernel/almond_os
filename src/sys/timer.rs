//! Provides Functions For Communicating With The Programmable Interrupt Timer
//! & General Sleep Functions

use bit_field::BitField;
use x86_64::instructions::port::Port;
use crate::{KResult, no_interrupt};
use super::interrupt::idt::set_irq_handler;
use spin::Mutex;

static mut TICK_COUNT: u64 = 0;


/// The Main PIT Freqency, Runs At 1.93810Mhz.
const MASTER_RATE: f64 = 1193810.0;
const COMMAND_PORT: u16 = 0x43;
const CH0_DATA: u16 = 0x40;
const SET_CH0_FREQ_CMD: u8 = 0x36;


/// Sets The Given PIT Channel's Frequency, Matching As Close As Possible.
pub fn set_frequency(freq: f64) {
    no_interrupt!({
        let divisor: usize = (MASTER_RATE / freq) as usize;
	    let actual: f64 = MASTER_RATE as f64 / divisor as f64;
	    let mut command_port: Port<u8> = Port::new(COMMAND_PORT);
	    let mut data_port: Port<u8> = Port::new(CH0_DATA);
	
	unsafe {
		command_port.write(SET_CH0_FREQ_CMD);
		data_port.write((divisor & 0xFF) as u8);
		data_port.write((divisor >> 8) as u8);
	}
    });



}

/// Setup The PIT, Set the frequency To 1000hz,
/// Set The IRQ0 Handler.
pub fn initialize() -> KResult<()> {
    no_interrupt!({
        set_frequency(5_0000.0);
        set_irq_handler(0, on_timer_tick);
    });
    Ok(())
}

#[doc(hidden)]
pub fn on_timer_tick(_: u8) {
    unsafe {
        TICK_COUNT += 1;
    }
}


/// Returns The Number Of Ticks Since Boot.
pub fn ticks() -> u64 {
    unsafe {
        TICK_COUNT
    }
}