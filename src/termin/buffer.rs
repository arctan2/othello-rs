use crossterm::style::{Color, Attribute};

#[derive(Clone)]
pub struct Cell {
  pub bg: Color,
  pub fg: Color,
  pub style: Attribute,
  pub symbol: String
}

impl Default for Cell {
  fn default() -> Self {
    Cell { bg: Color::Reset, fg: Color::Reset, style: Attribute::Reset, symbol: " ".into() }
  }
}

pub struct Rect {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16
}

impl Rect {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Rect { x, y, width, height }
  }

  pub fn area(&self) -> u16 {
    self.width * self.height
  }
}

impl Default for Rect {
  fn default() -> Self {
    Self::new(0, 0, 0, 0) 
  }
}

pub struct Buffer {
  rect: Rect,
  contents: Vec<Cell>
}

impl Buffer {
  pub fn empty(rect: Rect) -> Self {
    let a = rect.area() as usize;
    Buffer{ rect, contents: vec![Cell::default(); a] }
  }
  
  pub fn filled(rect: Rect, fill: Cell) -> Buffer {
    let a = rect.area() as usize;
    Buffer { rect, contents: vec![fill.clone(); a] }
  }
}
