use alloc::vec;

use crate::sys::storage::{ustar, almond_fs::block::Block};

use super::*;

pub struct HexDump;
pub struct SectorDump;

impl Program for HexDump {
    fn run(&mut self, args: Args) -> ShellExitCode {
        if args.len() >= 2 {
            let file = args[1].as_str();
            let md = ustar::MetaData::load(1, file);
            if let Some(md) = md {
                let mut buffer = vec![0; md.file_size() as usize]; 
                let bytes_read = md.read_data(&mut buffer);
                for row in (0..512).step_by(16) {
                    print!("${:04x} | ", row);
                    let mut text = String::new();
                    for col in 0..16 {
                        print!("{:02x} ", buffer[row + col]);
                        if (0x20..0x7f).contains(&buffer[row + col]) {
                            text.push(buffer[row + col] as char);
                        } else {
                            text.push('.');
                        }
                    }
                    print!("| {}\n", text);
        
                    if row > 0 && (row / 16) % 23 == 0 {input::input("(PRESS ENTER FOR MORE)");}
        
                }
            } else {
                print!("No Such File '{}'\n", file);
                return ShellExitCode::BadArguments;
            }
            return ShellExitCode::Ok;
        } else {
            print!("Usage: blkdump <input file>\n");
            return ShellExitCode::BadArguments;
        }
    }
}

impl Program for SectorDump {
    fn run(&mut self, args: Args) -> ShellExitCode {
        let addr: u32 = args[1].parse().unwrap_or(0);
        let buffer = Block::read(addr).unwrap();
        for row in (0..512).step_by(16) {
            print!("${:04x} | ", row);
            let mut text = String::new();
            for col in 0..16 {
                print!("{:02x} ", buffer[row + col]);
                if (0x20..0x7f).contains(&buffer[row + col]) {
                    text.push(buffer[row + col] as char);
                } else {
                    text.push('.');
                }
            }
            print!("| {}\n", text);

            if row > 0 && (row / 16) % 23 == 0 {input::input("(PRESS ENTER FOR MORE)");}

        }

        ShellExitCode::Ok
    }
}