use crossterm::style::Color;
use super::{Buffer, Rect, Element, impl_setters};

pub struct Text {
  rect: Rect,
  bg: Color,
  fg: Color,
  text: String
}

#[allow(dead_code)]
impl Text {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Self { rect: Rect::new(x, y, width, height), bg: Color::Reset, fg: Color::Reset, text: String::from("") }
  }

  pub fn fg(mut self, fg: Color) -> Self {
    self.fg = fg;
    self
  }

  pub fn set_fg(&mut self, fg: Color) {
    self.fg = fg;
  }

  pub fn text(mut self, text: &str) -> Self {
    self.text = text.to_string();
    if self.rect.width == 0 {
      self.rect.width = text.len() as u16;
    }
    if self.rect.height == 0 {
      self.rect.height = 1;
    }
    self
  }

  pub fn set_text(&mut self, text: &str) {
    self.text = text.to_string();
  }

  pub fn push_string(&mut self, s: String) {
    self.text.push_str(&s);
  }

  pub fn add_char_at(&mut self, idx: usize, ch: char) {
    self.text.insert(idx, ch)
  }

  pub fn remove_char_at(&mut self, idx: usize) {
    if self.text.len() == 0 {
      return;
    }
    self.text.remove(idx);
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }
}

impl_setters!(Text);

impl Default for Text {
  fn default() -> Self {
    Self::new(0, 0, 0, 0)
  }
}

impl Element for Text {
  fn draw(&self, buf: &mut Buffer) {
    let mut text = self.text.chars();

    for y in 0..self.rect.height {
      for x in 0..self.rect.width {
        let c = buf.get_mut(self.rect.x + x, self.rect.y + y);
        c.set_bg(self.bg);
        c.set_fg(self.fg);

        match text.next() {
          Some(sym) => c.set_symbol(sym),
          None => c.set_symbol(' ')
        }
      }
    }
  }
}