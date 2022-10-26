use crossterm::style::Color;

use super::buffer::{
  Buffer, Rect
};

pub trait Element {
  fn draw(&self, buf: &mut Buffer); 
}

pub struct Rectangle {
  rect: Rect,
  bg: Color
}

impl Rectangle {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Rectangle { rect: Rect::new(x, y, width, height), bg: Color::Reset }
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.rect.x = x;
    self.rect.y = y;
    self
  }

  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.rect.width = width;
    self.rect.height = height;
    self
  }

  pub fn bg(mut self, bg: Color) -> Self {
    self.bg = bg;
    self
  }

  pub fn set_bg(&mut self, bg: Color) {
    self.bg = bg;
  }

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.rect.x = x;
    self.rect.y = y;
  }

  pub fn set_size(&mut self, width: u16, height: u16) {
    self.rect.width = width;
    self.rect.height = height;
  }
}

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
