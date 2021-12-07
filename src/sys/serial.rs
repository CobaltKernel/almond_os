//! I/O Functions / Macros For Printing To The UART

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use core::fmt::{Arguments, Write};

use crate::no_interrupt;

const COMM0_ADDR: u16 = 0x3F8;

lazy_static! {
    static ref COMM0: Mutex<SerialPort> = unsafe { Mutex::new(SerialPort::new(COMM0_ADDR)) };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    no_interrupt!({
        COMM0.lock().write_fmt(args).expect("Failed To Print To Serial 0");
    });
}

/// Print To COMM 0
#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {
        $crate::sys::serial::_print(format_args!($($arg)*));
    }
}