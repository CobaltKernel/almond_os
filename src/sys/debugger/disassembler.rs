//! x64 Disassembler
use alloc::{format, string::String, vec};
use iced_x86::*;

use crate::{sys::mem::mapper::physical_memory_offset};

/// Disassemble x86 Instructions Into The NASM Format.
pub fn disassemble(addr: usize, len: usize) -> String {

    let addr = addr + physical_memory_offset() as usize;

    let mut bytes = vec![0; len];
    for idx in 0..len {
        let ptr = addr as *const u8;
        bytes[idx] = unsafe { ptr.offset(idx as isize).read() };
    }
    let mut decoder = Decoder::with_ip(64, bytes.as_ref(), addr as u64, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::default();
    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        let mut ins = String::new();
        formatter.format(&instruction, &mut ins);
        output.push_str(format!("0x{:08X} | ", decoder.ip() - instruction.len() as u64).as_str());
        output.push_str(format!("{} | ",ins).as_str());
        let start = (decoder.ip() as usize) - addr;
        //slog!("Start: {}\n", start);
        let mut end = start + instruction.len();
        //slog!("End: {}\n", end);
        if end >= bytes.len() {end = bytes.len();}
        let slice = &bytes[start..end];
        for b in slice.iter() {
            output.push_str(format!("{:02x} ", b).as_str())
        }
        output.push_str("\n")
    }

    output
}