//! Packages & Controls Long-Term Storage Mechanisms.

use crate::KResult;
use crate::sys;

pub mod ata;
pub mod sector;
pub mod fs_utils;


pub mod nut_fs;


/// Link IRQ 14 & 15 To The ATA Handlers.
pub fn initialize() -> KResult<()> {
    sys::interrupt::set_irq_handler(14, ata::bus_0_irq);
    sys::interrupt::set_irq_handler(15, ata::bus_1_irq);
    Ok(())
}