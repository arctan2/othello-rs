use std::io::{stdout, Write};
use crossterm::{
  queue,
  cursor::{MoveTo},
  style::{Print, Color, SetBackgroundColor, ResetColor},
  terminal::{Clear, ClearType},
  event::{read, Event, KeyCode}
};

#[derive(Copy, Clone)]
pub struct Coord {
  pub x: i32,
  pub y: i32
}

pub struct Window {
  pub start: Coord,
  pub height: i32,
  pub width: i32
}

impl Coord {
  pub fn new(x: i32, y: i32) -> Coord {
    Coord { x, y }
  }
}

impl Window {
  pub fn new_with_coord(coord_1: Coord, coord_2: Coord) -> Window {
    Window {
      start: coord_1,
      height: coord_2.y - coord_1.y,
      width: coord_2.x - coord_1.x,
    }
  }

  #[allow(dead_code)]
  pub fn new_with_dimen(start: Coord, height: i32, width: i32) -> Window {
    Window { start, height, width }
  }
}

pub trait WindowOperations {
  fn move_cursor(&self, x: u16, y: u16);
  fn print_string(&self, str: String);
  fn print_str(&self, str: &str);
  fn set_cell_bg(&self, x: u16, y: u16, color: Color);
  fn getch(&self) -> KeyCode;
  fn refresh(&self);
  fn clear(&self);
}

impl WindowOperations for Window {
  fn move_cursor(&self, x: u16, y: u16) {
    queue!(stdout(), MoveTo(x, y)).unwrap();
  }

  fn print_string(&self, str: String) {
    queue!(stdout(), Print(str)).unwrap();
  }

  fn print_str(&self, str: &str) {
    self.print_string(str.to_string());
  }

  fn set_cell_bg(&self, x: u16, y: u16, color: Color) {
    queue!(stdout(), MoveTo(x, y), SetBackgroundColor(color), Print(" "), ResetColor).unwrap();
  }

  fn clear(&self) {
    queue!(stdout(), Clear(ClearType::All)).unwrap();
  }

  fn refresh(&self) {
    stdout().flush().unwrap();
  }

  fn getch(&self) -> KeyCode {
    loop {
      match read() {
        Ok(Event::Key(event)) => { return event.code; }, 
        Err(_) => panic!("oopsi"),
        _ => continue
      }
    }
  }
}
