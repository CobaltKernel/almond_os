#![no_std]
#![deny(missing_docs)]
#![warn(missing_debug_implementations)]
#![feature(abi_x86_interrupt)]

//! Almond OS - Library

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

pub mod sys;

pub use x86_64::instructions::interrupts::without_interrupts;

/// The Kernel Result, Used To unify error-handling / reporting.
pub type KResult<T> = core::result::Result<T, &'static str>;

// TODO(George): Add Boot Code
/// Run Boot Code
pub fn boot() { 
    clear!();
    print!("Almond v{}\n", build_version!());
    strict_initialize!(test_init);
    strict_initialize!(sys::interrupt::initialize);

    x86_64::instructions::interrupts::int3();
}   

fn test_init() -> KResult<()> {
    Ok(())
}

/// Goes Into A Halt-Loop, Doesn't return. THERE IS NO ESCAPE...
pub fn halt() -> ! {
    loop {hlt()}
}

#[panic_handler]
#[doc(hidden)]
pub fn _panic(_: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
/// Utility To Run Intitialize Functions & Report Status To The User. Uses [KResult]
macro_rules! strict_initialize {
    // No-arg Version
    ($f:path) => {
        {
            $crate::print!("Initializing {} - ", stringify!($f));
            let result = $f();
            if result.is_err() {
                crate::eprint!("[FAILED]\n");
                halt();
            } else {
                crate::print!("[OK]\n");
            }
            result.unwrap()
        }
    };

    // One-Arg Version
    ($f:path, $arg_0:expr) => {
        {
            $crate::print!("Initializing {} - ", stringify!($f));
            let result = $f($arg_0);
            if result.is_err() {
                crate::eprint!("[FAILED]\n");
                halt();
            } else {
                crate::print!("[OK]\n");
            }
            result.unwrap()
        }
    };
}

#[macro_export] 
/// Runs The Given Code Inside A without_interrupts() call.
macro_rules! no_interrupt {
    ($code:block) => {
        $crate::without_interrupts(|| -> _ {
            $code
        })
    };
}

#[macro_export]
/// Returns The Build Version, As A str.
macro_rules! build_version {
    () => {
      {env!("CARGO_PKG_VERSION")}  
    };
}

#[macro_export]
/// Returns The Build Version, As A str.
macro_rules! build_name {
    () => {
      {env!("CARGO_PKG_NAME")}  
    };
}