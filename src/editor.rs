use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;
// use std::panic::{set_hook, take_hook};

mod terminal;
use terminal::Terminal;

mod view;
use view::{Direction, View};

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
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            _ => false,
        };

        if should_process {
            match event {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.quit = true,
                    (KeyCode::Up, _) => self.view.move_cursor(Direction::Up),
                    (KeyCode::Down, _) => self.view.move_cursor(Direction::Down),
                    (KeyCode::Left, _) => self.view.move_cursor(Direction::Left),
                    (KeyCode::Right, _) => self.view.move_cursor(Direction::Right),
                    // (KeyCode::Char(char), _) => println!("char: {:?}", char),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn refresh_screen(&self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to_pos(self.view.get_cursor_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::flush();
    }
}
