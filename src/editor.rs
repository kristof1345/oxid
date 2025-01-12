use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
// use std::panic::{set_hook, take_hook};

mod terminal;
use terminal::{Position, Terminal};

mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    quit: bool,
    view: View,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), io::Error> {
        Terminal::init()?;

        loop {
            self.refresh_screen();
            if self.quit {
                Terminal::terminate()?;
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
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.quit = true,
                // (KeyCode::Char(char), _) => println!("char: {:?}", char),
                _ => {}
            },
            _ => {}
        }
    }

    fn refresh_screen(&self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to_pos(Position { col: 0, row: 0 });
        let _ = Terminal::show_cursor();
        let _ = Terminal::flush();
    }
}
