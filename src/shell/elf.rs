use elf_rs::{Elf, ElfFile};

use crate::sys::storage::ustar;

use super::*;

pub struct ElfReader;

impl Program for ElfReader {
    fn run(&mut self, args: Args) -> ShellExitCode {
        let mut bytes = [0; 8192];
        ustar::load_bytes(&args[1], &mut bytes);
        let elf = Elf::from_bytes(&bytes).unwrap();
        print!("Section Headers:\n");
        for header in elf.section_header_iter() {
            print!("\t0x{:08x} (Align {}): {} - {:?}\n",
            header.addr(),
            header.addralign(), 
            String::from_utf8(header.section_name().to_vec()).expect("Failed To decode Name"),
            header.flags());
        }
        print!("Program Headers:\n");
        for header in elf.program_header_iter() {
            print!("\t0x{:08x} (Align {}): {} Bytes\n",
            header.vaddr(),
            header.align(),
            header.content().len(),
            );
        }

        return ShellExitCode::Ok;
    }
}