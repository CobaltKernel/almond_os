use super::*;

pub struct ClearScreen;


impl Program for ClearScreen {
    fn run(&mut self, _: Args) -> ShellExitCode {
        clear!(Color::Blue, Color::White);
        terminal::home();
        ShellExitCode::Ok
    }
}