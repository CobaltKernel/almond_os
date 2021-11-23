//! Front-end to the Interrupts Sub-System.
//! Attempts To Provide A Single, Stable API Across Platforms.

pub mod idt;
pub mod gdt;
pub mod tss;
pub mod pics;

use x86_64::instructions::interrupts;

use crate::{KResult, no_interrupt, print};


/// Abstracts An Interrupt Handler.
pub type InterruptHandler = fn(u8);

pub(self) const MAX_HANDLERS: usize = 256;

/// The Maximum Number Of Interrupt Handlers The Current Platform Supports.
#[inline(always)]
pub const fn max_handlers() -> usize {
    MAX_HANDLERS
}

/// Initialize The Interrupt System.
pub fn initialize() -> KResult<()> {
    unsafe {
        no_interrupt!({
            gdt::reload();
            idt::load();
            pics::init();
        });

        x86_64::instructions::interrupts::enable();
    }
    Ok(())
}

pub(self) fn default_handler(_: u8) {print!(".");}