use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{queue, Command};
use std::io::{self, stdout, Write};

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), io::Error> {
        Self::leave_alt_screen()?;
        Self::show_cursor()?;
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::enter_alt_screen()?;
        Self::clear_screen()?;
        Self::flush()?;
        Ok(())
    }

    pub fn move_cursor_to_pos(position: Position) -> Result<(), io::Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    fn enter_alt_screen() -> Result<(), io::Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    fn leave_alt_screen() -> Result<(), io::Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
