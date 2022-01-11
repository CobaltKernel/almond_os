use crate::sys::storage::ustar;

use super::*;

pub struct Cat;

impl Program for Cat {
    fn run(&mut self, args: Args) -> ShellExitCode {
        if args.len() >= 2 {
            let file = args[1].as_str();
            let md = ustar::MetaData::load(1, file);
            if let Some(md) = md {
                print!("{}\n", md.read_string());
            } else {
                print!("No Such File '{}'\n", file);
                return ShellExitCode::BadArguments;
            }
            return ShellExitCode::Ok;
        } else {
            print!("Usage: cat <input file>\n");
            return ShellExitCode::BadArguments;
        }
    }
}