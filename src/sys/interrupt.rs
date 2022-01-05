//! Front-end to the Interrupts Sub-System.
//! Attempts To Provide A Single, Stable API Across Platforms.

pub mod gdt;
pub mod idt;
pub mod pics;
pub mod tss;

use crate::{no_interrupt, print, KResult};

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

/// Set The IRQ handler Function.
pub fn set_irq_handler(irq: usize, handler: InterruptHandler) {
    idt::set_irq_handler(irq, handler)
}

pub(self) fn default_handler(_: u8) {
    print!(".");
}
