//! Provides Functions For Communicating With The Programmable Interrupt Timer
//! & General Sleep Functions

use super::interrupt::idt::set_irq_handler;
use crate::{no_interrupt, KResult};
use x86_64::instructions::{hlt, port::Port};

static mut TICK_COUNT: u64 = 0;
/// The Amount Of Ticks That Occur In One Second. 10KHz
pub const TICKS_PER_SECOND: f64 = 1000.6789606035205f64;

/// The Main PIT Freqency, Runs At 1.93810Mhz.
const MASTER_RATE: f64 = 1193810.0;
const COMMAND_PORT: u16 = 0x43;
const CH0_DATA: u16 = 0x40;
const CH2_DATA: u16 = 0x42;
const SET_CH0_FREQ_CMD: u8 = 0x36;
const SET_CH2_FREQ_CMD: u8 = 0xb6;

/// Sets The Given PIT Channel's Frequency, Matching As Close As Possible.
pub fn set_frequency_ch0(freq: f64) {
    no_interrupt!({
        let divisor: usize = (MASTER_RATE / freq) as usize;
        let mut command_port: Port<u8> = Port::new(COMMAND_PORT);
        let mut data_port: Port<u8> = Port::new(CH0_DATA);

        unsafe {
            command_port.write(SET_CH0_FREQ_CMD);
            data_port.write((divisor & 0xFF) as u8);
            data_port.write((divisor >> 8) as u8);
        }
    });
}

/// Sets The Given PIT Channel's Frequency, Matching As Close As Possible.
pub fn set_frequency_ch2(freq: f64) {
    no_interrupt!({
        let divisor: usize = (MASTER_RATE / freq) as usize;
        let mut command_port: Port<u8> = Port::new(COMMAND_PORT);
        let mut data_port: Port<u8> = Port::new(CH2_DATA);

        unsafe {
            command_port.write(SET_CH2_FREQ_CMD);
            data_port.write((divisor & 0xFF) as u8);
            data_port.write((divisor >> 8) as u8);
        }
    });
}

/// Setup The PIT, Set the frequency To 10KHz,
/// Set The IRQ0 Handler.
pub fn initialize() -> KResult<()> {
    no_interrupt!({
        set_frequency_ch0(TICKS_PER_SECOND);
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
    unsafe { TICK_COUNT }
}

/// Sleeps For An Amount Of Ticks. One Tick = 1ms
pub fn sleep_ticks(time: u64) {
    let start = ticks();
    loop {
        let now = ticks();
        if (now - start) >= time {
            break;
        }
        hlt();
    }
}

/// Uptime In Seconds
pub fn uptime() -> f64 {
    ticks() as f64 / TICKS_PER_SECOND
}
