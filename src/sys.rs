//! Holds System Functions.


use alloc::string::String;
pub mod interrupt;
pub mod mem;
pub mod serial;
pub mod sound;
pub mod storage;
pub mod terminal;
pub mod timer;
pub mod vga;
pub mod debugger;
pub mod input;
pub mod config;

static mut current_dir: String = String::new();

pub fn change_dir(path: &str) {
    unsafe {
        current_dir = String::from(path);
    }
}

pub fn dir() -> String {
    unsafe {current_dir.clone()}
}
