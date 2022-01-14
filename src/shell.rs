//! AlmondOS Shell Program
mod sleep;
mod debug;
mod ls;
mod beep;
mod env;
mod clear;
mod cat;
mod mount;
mod hexdump;
mod almond_vm;
mod assembler;
mod elf;
mod texteditor;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::sys::{input, terminal};
use crate::sys::vga::Color;
use crate::{print, set_bg, set_fg, clear, globals};

use self::assembler::Assembler;
use self::beep::Beep;
use self::cat::Cat;
use self::clear::ClearScreen;
use self::debug::{Disassemble, RegisterDump, MemoryDump};
use self::elf::ElfReader;
use self::hexdump::{HexDump, SectorDump};
use self::ls::FileLister;
use self::mount::Mount;
use self::sleep::Sleep;
use self::texteditor::TextEditor;

/// Alias For The Arguments Of A Program
pub type Args = Vec<String>;

/// Program Trait
pub trait Program {
    /// Run The Program
    fn run(&mut self, args: Args) -> ShellExitCode;
}

fn parse_cmd(cmd: String) -> Args {
    let v = cmd.split_whitespace().map(|s| {s.to_string()}).collect();
    v
}

/// Run The Given Command
pub fn run(cmd: &str) -> ShellExitCode {
    let parts: Vec<String> = parse_cmd(cmd.into());
    let code = match parts[0].as_str() {
        "sleep" => Sleep.run( parts),
        "disassemble" | ":d" => Disassemble.run(parts),
        "registers" | ":r" => {RegisterDump.run(parts)}
        "memory_dump" | ":md" => {MemoryDump.run(parts)}
        "ls" => {FileLister.run(parts)}
        "clear" | "clr" | "cls" => {ClearScreen.run(parts)}
        "beep" => {Beep.run(parts)}
        "cat" => {Cat.run(parts)}
        "mount" => {Mount.run(parts)}
        "hexdump" => {HexDump.run(parts)}
        "blkdump" | "blkd" => {SectorDump.run(parts)}
        "asm" => {Assembler::get(parts.clone()).run(parts)}
        "elf" => {ElfReader.run(parts)}

        "ted" => {TextEditor::load_or_create(parts.clone()).run(parts)}

        _ => { print!("Unknown Command: '{}'...\n", cmd); ShellExitCode::NoSuchProgram},
    };

    return code;
}

/// Run The Shell Environment
pub fn main() {
    terminal::home();
    clear!(Color::Blue, Color::White);
    set_bg!(Color::Blue);
    set_fg!(Color::White);
    'input_loop: loop {
        let  cmd = input::input(">> ");
        if cmd.is_empty() {continue 'input_loop;}
        if cmd == String::from("exit") {break 'input_loop;}
        run(cmd.as_str());
    }

    terminal::home();
}





/// Program return Codes.
#[derive(Debug)]
pub enum ShellExitCode {
    /// The Program Exited Properly.
    Ok = 0,
    /// The Program Received Bad / Incorrect Arguments.
    BadArguments = 1,
    /// The Program Attempted To Perform A Privledged Action, Without The Necessary Permissions.
    PrivledgeError = 2,

    /// The Program Was Not Found
    NoSuchProgram = 16,
}

#[macro_export]
/// Run A Shell Command
macro_rules! run {
    ($fmt:expr, $($args:tt)*) => {
        {
            let cmd = alloc::format!($fmt, $($args)*);
            $crate::shell::run(cmd.as_str());
        }
    };


    ($fmt:expr) => {
        {
            let cmd = alloc::format!($fmt);
            $crate::shell::run(cmd.as_str());
        }
    };
}