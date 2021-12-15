use crate::sys::timer::sleep_ticks;

use super::{Args, Program, ShellExitCode};

pub struct Sleep;

impl Program for Sleep {
    fn run(&mut self, args: Args) -> ShellExitCode {
        let amount = args[1].parse().unwrap_or(1);
        sleep_ticks(amount);
        ShellExitCode::Ok
    }
}