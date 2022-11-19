use crossterm::style::Color;
use super::{
  Buffer, Rect, Element, impl_setters, Position
};

#[derive(Debug)]
pub struct Rectangle {
  rect: Rect,
  bg: Color
}

#[allow(dead_code)]
impl Rectangle {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Rectangle { rect: Rect::new(x, y, width, height), bg: Color::Reset }
  }

  pub fn from_rect(rect: Rect) -> Self {
    let mut r = Self::default();
    r.rect = rect;
    r
  }

  pub fn bg(mut self, bg: Color) -> Self {
    self.bg = bg;
    self
  }

  pub fn set_bg(&mut self, bg: Color) {
    self.bg = bg;
  }
}

impl_setters!(Rectangle);

impl Default for Rectangle {
  fn default() -> Self {
    Self::new(0, 0, 0, 0)
  }
}

impl Element for Rectangle {
  fn draw(&self, buf: &mut Buffer) {
    for y in 0..self.rect.height {
      for x in 0..self.rect.width {
        let c = buf.get_mut(self.rect.x + x, self.rect.y + y);
        c.set_bg(self.bg);
      }
    }
  }
}
