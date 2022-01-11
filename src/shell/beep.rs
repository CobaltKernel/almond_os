use crate::sys::{sound::pc_speaker, timer::sleep_ticks};

use super::Program;

pub struct Beep;

impl Program for Beep {
    fn run(&mut self, args: super::Args) -> super::ShellExitCode {

        if args.len() >= 3 {

            pc_speaker::set_pitch(args[1].parse().expect("Expected A Number."));
            pc_speaker::play();
            sleep_ticks(args[2].parse().expect("Expected An Integer."));
            pc_speaker::stop();
        } else {
            return super::ShellExitCode::BadArguments;
        }

        return super::ShellExitCode::Ok;
    }
}