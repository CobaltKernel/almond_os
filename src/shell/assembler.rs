use alloc::collections::{BTreeSet, BTreeMap};

use crate::{slog, sys::storage::ustar::MetaData, sprint};

use super::*;

const INSTRUCTION_LENGTH: usize = 4;

enum Token {
    Instruction(String),
    LiteralNumber(usize),
    LiteralString(String)
}

pub struct Assembler {
    src: Vec<String>,
    symbols: BTreeMap<String, usize>,
    output: Vec<u8>,
}

impl Assembler {

    pub fn get(args: Args) -> Self {
        let file = MetaData::load(1, &args[1]).unwrap();
        return Self::new(file.read_string());
    }

    pub fn new(src: String) -> Self {
        Self {
            output: Vec::new(),
            src: src.lines().map(|s| -> String {s.to_string()}).collect(),
            symbols: BTreeMap::new(),
        }
    }

    pub fn resolve_symbols(&mut self) {
        let mut address = 0;
        for (index, line) in self.src.iter_mut().enumerate() {
            *line = String::from(line.trim());
            if line.is_empty() { continue; }
            
            if line.ends_with(":") {
                self.symbols.insert(line.strip_suffix(":").unwrap().to_string(), address);
                slog!("Found Label '{}'\n", line);
            }

            address += INSTRUCTION_LENGTH;
        }

        slog!("Found {} Symbols:\n", self.symbols.len());
        for key in self.symbols.keys() {
            sprint!("\t {}: {:04x} \n", key, self.symbols.get(key).unwrap_or( &(u16::MAX as usize)))
        }

    }
}

impl Program for Assembler {

    fn run(&mut self, args: Args) -> ShellExitCode {
        self.resolve_symbols();

        ShellExitCode::Ok
    }
}