use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
// use std::panic::{set_hook, take_hook};

#[derive(Default)]
pub struct Editor {
    quit: bool,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;

        loop {
            if self.quit {
                disable_raw_mode()?;
                break;
            }
            match read() {
                Ok(event) => self.eval_events(event),
                Err(err) => panic!("{err}"),
            }
        }

        Ok(())
    }

    pub fn eval_events(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(char) => {
                    if char == 'q' {
                        self.quit = true;
                    }
                    println!("{:?}", char);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
