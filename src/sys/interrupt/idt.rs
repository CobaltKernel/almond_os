//! Utility Functions For Interacting With IDT.

use crate::no_interrupt;
use super::{InterruptHandler, MAX_HANDLERS, default_handler};
use lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

/// The Offset Of The PIC1
/// handler_index = irq + PIC1;
pub const PIC1_OFFSET: usize = 0x20;
/// The Offset Of The PIC2.
pub const PIC2_OFFSET: usize = 0x28;

lazy_static::lazy_static! {
    static ref HANDLERS: Mutex<[InterruptHandler; MAX_HANDLERS]> = Mutex::new([default_handler; MAX_HANDLERS]);

    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt[system_index(0)].set_handler_fn(irq_0);
        idt[system_index(1)].set_handler_fn(irq_1);
        idt[system_index(2)].set_handler_fn(irq_2);
        idt[system_index(3)].set_handler_fn(irq_3);
        idt[system_index(14)].set_handler_fn(irq_14);
        idt[system_index(15)].set_handler_fn(irq_15);

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt
    };
}

/// Load The IDT Values Into The IDT.
pub unsafe fn load() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    crate::print!("EXCEPTION: BREAKPOINT\n{:#?}\n", stack_frame);
}

/// Set The Handler Function For A Given IRQ.
pub fn set_irq_handler(irq: usize, handler: InterruptHandler) {
    no_interrupt!({
        //crate::print!("Handler Set To {:p} from {:p}\n", handler as *const u8, default_handler as *const u8);
        let mut handlers = HANDLERS.lock();
        handlers[system_index(irq)] = handler;
        //crate::print!("Handler Set To {:p} from {:p}\n", handlers[system_index(irq)] as *const u8, default_handler as *const u8);
    });
}

/// Converts A IRQ Into A System Interrupt Index
pub fn system_index(irq: usize) -> usize {
    irq + PIC1_OFFSET
}

#[macro_export]
/// Generate An IRQ Handler for the given IRQ.
macro_rules! gen_irq {
    ($handler:ident, $irq:expr) => {
        /// PRE-GENERATED IRQ HANDLER
        pub extern "x86-interrupt" fn $handler(_stack_frame: InterruptStackFrame) {
            let handlers = HANDLERS.lock();
            handlers[system_index($irq)]($irq);
            unsafe { crate::sys::interrupt::pics::PICS.lock().notify_end_of_interrupt(system_index($irq) as u8); }
        }
    };
}

gen_irq!(irq_0, 0);
gen_irq!(irq_1, 1);
gen_irq!(irq_2, 2);
gen_irq!(irq_3, 3);
gen_irq!(irq_4, 4);
gen_irq!(irq_5, 5);
gen_irq!(irq_14, 14);
gen_irq!(irq_15, 15);