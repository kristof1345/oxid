use super::terminal::{Position, Size, Terminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Copy, Clone)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Default)]
pub struct View {
    size: Size,
    location: Location,
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

    pub fn get_cursor_position(&self) -> Position {
        Position {
            col: self.location.x,
            row: self.location.y,
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

    pub fn move_cursor(&mut self, direction: Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;

        match direction {
            Direction::Up => {
                if y != 0 {
                    y = y.saturating_sub(1);
                }
            }
            Direction::Down => {
                if y < height - 1 {
                    y = y.saturating_add(1);
                }
            }
            Direction::Right => {
                if x < width - 1 {
                    x = x.saturating_add(1);
                }
            }
            Direction::Left => {
                if x != 0 {
                    x = x.saturating_sub(1);
                }
            }
        }

        self.location = Location { x, y };
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
        }
    }
}
