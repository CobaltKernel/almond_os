use alloc::format;

use crate::sys::{storage::mfs::{self, file::{File, SeekFrom}, api::FileIO}, timer, input::{DELETE, BACKSPACE}};

use super::*;

pub struct TextEditor {
    text: String, 
    file: File,
}

impl TextEditor {
    pub fn load_or_create(args: Args) -> TextEditor {
        let mut text: String = String::new();
        let mut file: Option<File> = None; 

        let path = args.get(1).unwrap();

        if let Some(mut fle) = mfs::open_file(&path) {
            text = fle.read_to_string();
            file = Some(fle);
        } else {
            file = mfs::create_file(&path);
        }

        Self {file: file.unwrap(), text}
    }

}

impl Program for TextEditor {
    fn run(&mut self, args: Args) -> ShellExitCode {
        clear!(Color::Blue, Color::White);
        terminal::home();
        terminal::put_string(0, 0, &format!("TED - {}", self.file.name()), (Color::Blue, Color::White));
        terminal::put_string(0,1, &self.text, (Color::White, Color::Blue));
        while(true) {
            if let Some(chr) = input::read_key() {
                match chr {
                    BACKSPACE => {self.text.pop();}
                    DELETE => {self.file.seek(SeekFrom::Start(0));self.file.write(self.text.as_bytes());  break;}
                    _ => {self.text.push(chr)}
                }
                clear!(Color::Blue, Color::White);
                terminal::home();
                terminal::put_string(0, 0, &format!("TED - {}", self.file.name()), (Color::Blue, Color::White));
                terminal::put_string(0,1, &self.text, (Color::White, Color::Blue));


                terminal::put_string(0, 24, "Press DEL To Exit...", (Color::Blue, Color::White));

            } else {
                timer::sleep_ticks(16);
            }

            
            
        }

        ShellExitCode::Ok
    }
}


