#![no_std]
//#![deny(missing_docs)]
#![warn(missing_debug_implementations)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(asm)]

//! Almond OS - Library

extern crate alloc;

use alloc::string::String;
use bootloader::BootInfo;
use sys::config::SystemConfig;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

pub mod sys;
pub mod shell;
pub mod vfs;

pub use x86_64::instructions::interrupts::without_interrupts;

use crate::{sys::storage::ustar, vfs::{Device, AtaDevice}};

/// The Kernel Result, Used To unify error-handling / reporting.
pub type KResult<T> = core::result::Result<T, &'static str>;

lazy_static! {
    pub static ref GLOBAL_CONFIG: Mutex<Option<SystemConfig>> = Mutex::new(None);
}

// TODO(George): Add Boot Code
/// Run Boot Code
pub fn boot(info: &'static BootInfo) {
    clear!();
    log!("Almond v{}\n", build_version!());
    strict_initialize!(test_init);
    strict_initialize!(sys::interrupt::initialize);
    strict_initialize!(sys::timer::initialize);
    strict_initialize!(sys::input::initialize);
    strict_initialize!(sys::mem::initialize, info);
    strict_initialize!(sys::storage::initialize);

    run!("mount HDB");

    if sys::storage::ustar::list(1).contains(&String::from("disk/global.cfg")) {
        slog!("Loading Global Config\n");
        let mut cfg_lock =  GLOBAL_CONFIG.lock();
        *cfg_lock = Some(SystemConfig::from_str(&ustar::MetaData::load(1, "disk/global.cfg").unwrap().read_string()));
    } else {
        serr!("Unable To Locate 'disk/global.cfg'\n");
    }
}

fn test_init() -> KResult<()> {
    Ok(())
}

/// Goes Into A Halt-Loop, Doesn't return. THERE IS NO ESCAPE...
pub fn halt() -> ! {
    loop {
        hlt()
    }
}

#[panic_handler]
#[doc(hidden)]
pub fn _panic(info: &PanicInfo) -> ! {
    print!("Panic: {}", info);
    serr!("Panic: {}", info);
    loop {}
}

#[macro_export]
/// Utility To Run Intitialize Functions & Report Status To The User. Uses [KResult]
macro_rules! strict_initialize {
    // No-arg Version
    ($f:path) => {{
        $crate::log!("Initializing {} - ", stringify!($f));
        let result = $f();
        if result.is_err() {
            crate::eprint!("[FAILED]\n");
            halt();
        } else {
            $crate::set_fg!($crate::sys::vga::Color::Green);
            crate::print!("[OK]\n");
            $crate::set_fg!($crate::sys::vga::Color::White);
        }
        result.unwrap()
    }};

    // One-Arg Version
    ($f:path, $arg_0:expr) => {{
        $crate::log!("Initializing {} - ", stringify!($f));
        let result = $f($arg_0);
        if result.is_err() {
            crate::eprint!("[FAILED]\n");
            halt();
        } else {
            $crate::set_fg!($crate::sys::vga::Color::Green);
            crate::print!("[OK]\n");
            $crate::set_fg!($crate::sys::vga::Color::White);
        }
        result.unwrap()
    }};
}

#[macro_export]
/// Runs The Given Code Inside A without_interrupts() call.
macro_rules! no_interrupt {
    ($code:block) => {
        $crate::without_interrupts(|| -> _ { $code })
    };
}

#[macro_export]
/// Returns The Build Version, As A str.
macro_rules! build_version {
    () => {{
        env!("CARGO_PKG_VERSION")
    }};
}

#[macro_export]
/// Returns The Build Version, As A str.
macro_rules! build_name {
    () => {{
        env!("CARGO_PKG_NAME")
    }};
}

/// Wrapper Type Around [spin::Mutex]
#[derive(Debug)]
pub struct Locked<T> {
    inner: spin::Mutex<T>,
}

impl<T> Locked<T> {
    /// Wraps inner in a Mutex
    pub const fn new(inner: T) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }

    /// Locks inner & Returns The Guard.
    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

#[macro_export]
/// Logs To The Terminal
macro_rules! log {
    ($fmt:expr, $($arg:tt)*) => {
        $crate::set_fg!($crate::sys::vga::Color::Green);
        $crate::print!("[LOG]: ");
        $crate::set_fg!($crate::sys::vga::Color::Yellow);
        $crate::print!($fmt, $($arg)*);
        $crate::set_fg!($crate::sys::vga::Color::White);
    };

    ($fmt:expr) => {
        $crate::set_fg!($crate::sys::vga::Color::Green);
        $crate::print!("[LOG]: ");
        $crate::set_fg!($crate::sys::vga::Color::Yellow);
        $crate::print!($fmt);
        $crate::set_fg!($crate::sys::vga::Color::White);
    };
}

#[macro_export]
/// Logs To The Terminal
macro_rules! err {
    ($fmt:expr, $($arg:tt)*) => {

        $crate::eprint!(concat!("[ERR]: ", $fmt), $($arg)*);
    };

    ($fmt:expr) => {
        $crate::eprint!(concat!("[ERR]: ", $fmt));
    };
}

#[macro_export]
/// Log To The Serial Port
macro_rules! slog {
    ($fmt:expr, $($arg:tt)*) => {
        $crate::sprint!(concat!("\x1b[32m[LOG|", file!(), ":", line!(), ":", column!(), "]: \x1b[39m",  $fmt), $($arg)*);
    };

    ($fmt:expr) => {
        $crate::sprint!(concat!("\x1b[32m[LOG|", file!(), ":", line!(), ":", column!(), "]: \x1b[39m",  $fmt));
    };

}

#[macro_export]
/// Log To The Serial Port
macro_rules! serr {
    ($fmt:expr, $($arg:tt)*) => {
        $crate::sprint!(concat!("\x1b[31m[ERR]:\x1b[39m ", $fmt), $($arg)*);
    };

    ($fmt:expr) => {
        $crate::sprint!(concat!("\x1b[31m[ERR]:\x1b[39m ", $fmt));
    };

}

/// Wait For The Next Interrupt.
pub fn spin() {
    hlt()
}

/// Lock The System Globals Config.
pub fn globals<'a>() -> MutexGuard<'a, Option<SystemConfig>> {
    GLOBAL_CONFIG.lock()
}