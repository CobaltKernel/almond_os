//! Utilities For The 8259 PICs.
//! Thanks To: https://github.com/vinc/moros/blob/trunk/src/sys/pic.rs

use pic8259::ChainedPics;
use spin::Mutex;

use super::idt::{PIC1_OFFSET, PIC2_OFFSET};
/// Represents The two 8259 Programmable Interrupt Controllers present in the x86 Architecture.
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET as u8, PIC2_OFFSET as u8) });

/// Initialize The PICS.
pub fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}