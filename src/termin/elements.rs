use super::buffer::{
  Buffer, Rect
};

pub trait Element {
  fn draw(&self, buf: &mut Buffer); 
}

pub struct Rectangle {
  rect: Rect
}

impl Rectangle {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Rectangle { rect: Rect::new(x, y, width, height) }
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
}

impl Default for Rectangle {
  fn default() -> Self {
    Self::new(0, 0, 0, 0)
  }
}

impl Element for Rectangle {
  fn draw(&self, buf: &mut Buffer) {
  }
}
