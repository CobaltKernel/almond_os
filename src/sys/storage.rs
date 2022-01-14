//! Packages & Controls Long-Term Storage Mechanisms.

use crate::sys;
use crate::KResult;

pub mod ata;
pub mod fs_utils;
pub mod sector;
pub mod almond_fs;
pub mod ustar;
pub mod nut_fs;
pub mod block;

pub mod mfs;

/// Link IRQ 14 & 15 To The ATA Handlers.
pub fn initialize() -> KResult<()> {
    sys::interrupt::set_irq_handler(14, ata::bus_0_irq);
    sys::interrupt::set_irq_handler(15, ata::bus_1_irq);
    Ok(())
}
