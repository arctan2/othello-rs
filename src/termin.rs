use std::io::stdout;
use crossterm::{
  execute,
  terminal::{
    LeaveAlternateScreen, EnterAlternateScreen, size, enable_raw_mode, disable_raw_mode
  }
};
use window::{Window, Coord};

pub mod window;

pub fn init_termin() -> Window {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen).unwrap();
  let (w, h) = size().unwrap();
  Window::new_with_coord(Coord::new(0, 0), Coord::new(w as i32, h as i32))
}

pub fn end_termin() {
  disable_raw_mode().unwrap();
  execute!(stdout(), LeaveAlternateScreen).unwrap();
}