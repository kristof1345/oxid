use super::terminal::{Size, Terminal};
use std::io;

// #[derive(Default)]
pub struct View {
    size: Size,
}

impl View {
    pub fn render(&self) {
        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return;
        }

        for row in 0..height {
            let _ = Terminal::print_line("~");

            if row < height - 1 {
                let _ = Terminal::print_line("\r\n");
            }
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
