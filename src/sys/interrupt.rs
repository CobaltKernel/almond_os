//! Front-end to the Interrupts Sub-System.
//! Attempts To Provide A Single, Stable API Across Platforms.

pub mod idt;
pub mod gdt;
pub mod tss;
pub mod pics;

use crate::{KResult, print};


/// Abstracts An Interrupt Handler.
pub type InterruptHandler = fn();

pub(self) const MAX_HANDLERS: usize = 256;

/// The Maximum Number Of Interrupt Handlers The Current Platform Supports.
#[inline(always)]
pub const fn max_handlers() -> usize {
    MAX_HANDLERS
}

/// Initialize The Interrupt System.
pub fn initialize() -> KResult<()> {
    unsafe {
        gdt::reload();
        idt::load();
        pics::init();
    }
    Ok(())
}

pub(self) fn default_handler() {}