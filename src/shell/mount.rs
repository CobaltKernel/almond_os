use crate::vfs::{self, AtaDevice, Device};

use super::*;

pub struct Mount;

impl Program for Mount {
    fn run(&mut self, args: Args) -> ShellExitCode {
        if args.len() >= 2 {
            match args[1].as_str() {
                "HDA" => {vfs::mount(Device::ata(0))},
                "HDB" => {vfs::mount(Device::ata(1))},
                "HDC" => {vfs::mount(Device::ata(2))},
                "HDD" => {vfs::mount(Device::ata(3))},
                _ => {
                    print!("Unknown Disk: '{}', Expected HDA, HDB, HDC, HDD\n", args[1]);
                    return ShellExitCode::BadArguments;
                }
            }
            return ShellExitCode::Ok;
        } else {
            return ShellExitCode::BadArguments;
        }
    }
}