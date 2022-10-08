pub trait Element {
  fn draw(&self); 
}

pub struct Rectangle {
  x: u16,
  y: u16,
  width: u16,
  height: u16,
}

impl Rectangle {
  pub fn new(width: u16, height: u16, x: u16, y: u16) -> Self {
    Rectangle { x, y, width, height }
  }

  pub fn default() -> Self {
    Rectangle { x: 0, y: 0, width: 0, height: 0 }
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.x = x;
    self.y = y;
    self
  }

  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.width = width;
    self.height = height;
    self
  }
}

impl Element for Rectangle {
  fn draw(&self) {
  }
}