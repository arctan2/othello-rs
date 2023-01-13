use std::io::{Write, stdout};

use crossterm::{style::{Color, Attribute}, event::{KeyCode, Event}, execute, cursor, queue};

use crate::termin::{buffer::{Rect, Buffer, Cell}, window::{WindowRef, Window, Position}, crossterm_handler::CrosstermHandler};

use super::{Text, Element};

#[derive(Debug)]
pub struct InputBox {
  start_text: (u16, u16),
  width: u16,
  height: u16,
  x: u16, y: u16,
  max_len: i32,
  cursor: (u16, u16)
}

impl Default for InputBox {
  fn default() -> Self {
    Self { start_text: (0, 0), cursor: (0, 0), max_len: 0, width: 0, height: 0, x: 0, y: 0 }
  }
}

impl InputBox {
  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.width = width;
    self.height = height;
    self
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.x = x;
    self.y = y;
    self
  }

  pub fn max_len(mut self, max_len: i32) -> Self{
    self.max_len = max_len;
    self
  }

  pub fn start_text(mut self, start_text: (u16, u16)) -> Self {
    self.start_text = start_text;
    self
  }

  pub fn cursor_xy(&self, width: u16, idx: usize) -> (u16, u16) {
    let idx = idx as u16;
    (idx % width, idx / width)
  }

  pub fn read_string<W: Write>(&mut self, win: &mut WindowRef, handler: &mut CrosstermHandler<W>) -> String {
    execute!(stdout(), cursor::Show).unwrap();
    let mut cursor_pos: usize = 0;
    let mut input_win = win.new_child(
      Window::default().xy(self.x, self.y).size(self.width, self.height)
    );
    let mut input_box = Text::default().size(self.width, self.height);

    let (abs_y, abs_x) = input_win.abs_pos();

    handler.draw_window(&input_win).unwrap();
    let (rel_x, rel_y) = self.cursor_xy(input_win.width(), cursor_pos);
    queue!(stdout(), cursor::MoveTo(abs_x + rel_x, abs_y + rel_y)).unwrap();
    handler.flush().unwrap();

    loop {
      match handler.event() {
        Event::Key(k) => {
          match k.code {
            KeyCode::Esc => {
              execute!(stdout(), cursor::Hide).unwrap();
              input_win.delete();
              return input_box.get_text().to_string();
            },
            KeyCode::Enter => {
              execute!(stdout(), cursor::Hide).unwrap();
              input_win.delete();
              return input_box.get_text().to_string();
            },
            KeyCode::Backspace => {
              if cursor_pos > 0 {
                cursor_pos -= 1;
                input_box.remove_char_at(cursor_pos);
              }
            },
            KeyCode::Char(ch) => {
              if (input_box.get_text().len() as i32) < self.max_len {
                input_box.add_char_at(cursor_pos, ch);
                cursor_pos += 1;
              }
            },
            KeyCode::Left => {
              cursor_pos -= if cursor_pos != 0 { 1 } else { 0 };
            },
            KeyCode::Right => {
              cursor_pos += if cursor_pos == input_box.get_text().len() { 0 } else { 1 };
            },
            _ => ()
          }
        },
        _ => ()
      }

      input_win.clear();
      input_win.draw_element(&input_box);
      handler.draw_window(&input_win).unwrap();

      let (rel_x, rel_y) = self.cursor_xy(input_win.width(), cursor_pos);
      queue!(stdout(), cursor::MoveTo(abs_x + rel_x, abs_y + rel_y)).unwrap();
      handler.flush().unwrap();
    }
  }
}
