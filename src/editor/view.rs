use super::terminal::{Size, Terminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
            if row == height / 3 {
                Self::render_line(row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }
    }

    fn build_welcome_message(width: usize) -> String {
        let n_a_v = format!("Oxid -- version {}", VERSION);
        let padding = (width - n_a_v.len()) / 2;

        let msg = format!("~{}{}", " ".repeat(padding.saturating_sub(1)), n_a_v);

        msg
    }

    fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
