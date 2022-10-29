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
    self
  }

  pub fn set_text(&mut self, text: &str) {
    self.text = text.to_string();
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
          None => ()
        }
      }
    }
  }
}