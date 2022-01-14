use crate::sys::storage;

use super::*;

pub struct FileLister;

impl Program for FileLister {
    fn run(&mut self, _args: Args) -> ShellExitCode {

        for filename in storage::ustar::list(1) {
            print!(" - {}\n", filename);
        }

        ShellExitCode::Ok
    }
}