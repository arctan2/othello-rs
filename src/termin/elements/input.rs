use std::io::Write;

use crossterm::{style::Color, event::{KeyCode, Event}};

use crate::termin::{buffer::{Rect, Buffer}, window::WindowRef, crossterm_handler::CrosstermHandler};

use super::{Text, Element};

pub struct InputBox {
  input_box: Text,
  cursor: (u16, u16)
}

impl Default for InputBox {
  fn default() -> Self {
    Self { input_box: Text::default(), cursor: (0, 0) }
  }
}

impl InputBox {
  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.input_box.set_size(width, height);
    self
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.input_box.set_pos(x, y);
    self
  }

  pub fn start_text(mut self, start_text: (u16, u16)) -> Self {
    self.input_box = self.input_box.start_text(start_text);
    self
  }

  pub fn read_string<W: Write>(&mut self, win: &mut WindowRef, handler: &mut CrosstermHandler<W>) -> String {
    let mut cursor_pos: usize = 0;
    handler.draw_window(&win).unwrap();
    handler.flush().unwrap();

    loop {
      match handler.event() {
        Event::Key(k) => {
          match k.code {
            KeyCode::Esc => {
              return self.input_box.get_text().to_string();
            },
            KeyCode::Backspace => {
              if cursor_pos > 0 {
                cursor_pos -= 1;
                self.input_box.remove_char_at(cursor_pos);
              }
            },
            KeyCode::Char(ch) => {
              self.input_box.add_char_at(cursor_pos, ch);
              cursor_pos += 1;
            },
            KeyCode::Left => {
              cursor_pos -= if cursor_pos != 0 { 1 } else { 0 };
            },
            KeyCode::Right => {
              cursor_pos += if cursor_pos == self.input_box.get_text().len() { 0 } else { 1 };
            },
            _ => ()
          }
        },
        _ => ()
      }
      win.draw_element(&self.input_box);
      handler.draw_window(&win).unwrap();
      handler.flush().unwrap();
    }
  }
}
