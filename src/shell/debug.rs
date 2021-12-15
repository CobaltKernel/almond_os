//! Debugging Shell Commands
use crate::shell::ShellExitCode;
use crate::sys::debugger::{disassembler, self};
use crate::print;
use crate::sys::mem;

use super::*;


/// Disassemble Instructions
pub struct Disassemble;

impl Program for Disassemble {
    fn run(&mut self, args: Args) -> ShellExitCode {
        if args.len() < 3 {return ShellExitCode::BadArguments};
        let addr = usize::from_str_radix(&args[1], 16).unwrap_or(1);
        let len = usize::from_str_radix(&args[2], 10).unwrap_or(1);

        let output = disassembler::disassemble(addr, len);

        print!("{}\n", output);

        ShellExitCode::Ok
    }
}

pub struct RegisterDump;

impl Program for RegisterDump {
    fn run(&mut self, _: Args) -> ShellExitCode {

        print!("EAX: {:08x} - ECX: {:08x} - EBX: {:08x}\n",
            debugger::read_eax(), 
            debugger::read_ecx(), 
            debugger::read_ebx());

        print!("RIP: {:08x}\n", debugger::read_rip());
        print!("Cr0 Flags: {:?}\n", debugger::read_cr0());
        print!("Cr0: 0b{:064b}\n", debugger::read_cr0_raw());
        print!("Cr2 Flags: {:?}\n", debugger::read_cr2());
        print!("Cr2: 0b{:064b}\n", debugger::read_cr2_raw());
        print!("Cr3 Flags: {:?}\n", debugger::read_cr3());
        print!("Cr3: 0x{:016X}\n", debugger::read_cr3_raw());
        print!("Cr4 Flags: {:?}\n", debugger::read_cr4());
        print!("Cr4: 0b{:064b}\n", debugger::read_cr4_raw());
        ShellExitCode::Ok
    }
}

// Print Out Virtual Memory Values
pub struct MemoryDump;

impl Program for MemoryDump {
    fn run(&mut self, args: Args) -> ShellExitCode {
        if args.len() < 2 {return ShellExitCode::BadArguments};

        let base = usize::from_str_radix(args[1].split("..").nth(0).unwrap_or("0"), 16).unwrap();
        let end =  usize::from_str_radix(args[1].split("..").nth(1).unwrap_or("0"), 16).unwrap();
        for row in (base..end).step_by(16) {
            for col in 0..16 {
                
                print!("{:02x} ", unsafe { ((row + col) as *const u8).read() });
            }
            print!("\n");
        }
        ShellExitCode::Ok
    }
}