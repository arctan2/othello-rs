use std::{fmt, vec, rc::Rc};

use crossterm::style::{Color, Attribute};

#[derive(Clone, Debug)]
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

impl Cell {
  pub fn set_bg(&mut self, bg: Color) {
    self.bg = bg;
  }

  pub fn set_fg(&mut self, fg: Color) {
    self.fg = fg;
  }

  pub fn set_symbol(&mut self, sym: char) {
    self.symbol = sym.to_string();
  }

  pub fn set_style(&mut self, attr: Attribute) {
    self.style = attr;
  }

  pub fn reset(&mut self) {
    self.bg = Color::Reset;
    self.fg = Color::Reset;
    self.symbol = String::from(" ");
    self.style = Attribute::Reset;
  }
}

#[derive(Debug)]
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

impl fmt::Debug for Buffer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Buffer{{rect: {:?}, contents: Vec<Cell, {}>}}", self.rect, self.contents.len())
  }
}

impl Buffer {
  pub fn empty(rect: Rect) -> Self {
    let a = rect.area() as usize;
    Buffer{ rect, contents: vec![Cell::default(); a] }
  }

  pub fn contents_mut(&mut self) -> &mut Vec<Cell> {
    &mut self.contents
  }
  
  pub fn contents(&self) -> &Vec<Cell> {
    &self.contents
  }

  pub fn reset(&mut self) {
    for c in &mut self.contents {
      c.reset();
    }
  }

  pub fn width(&self) -> u16 {
    self.rect.width
  }

  pub fn height(&self) -> u16 {
    self.rect.height
  }
  
  pub fn filled(rect: Rect, fill: Cell) -> Buffer {
    let a = rect.area() as usize;
    Buffer { rect, contents: vec![fill.clone(); a] }
  }

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.rect.x = x;
    self.rect.y = y;
  }

  pub fn index_of(&self, x: u16, y: u16) -> usize {
    ((self.rect.width * y) + x) as usize
  }

  pub fn size(&self) -> usize {
    self.contents.len()
  }

  pub fn get(&self, x: u16, y: u16) -> &Cell {
    let idx = self.index_of(x, y);
    &self.contents[idx]
  }

  pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
    let idx = self.index_of(x, y);
    &mut self.contents[idx]
  }

  pub fn set_bg(&mut self, bg: Color) {
    for c in &mut self.contents {
      c.set_bg(bg);
    }
  }

  pub fn top(&self) -> u16 {
    self.rect.y
  }
  
  pub fn left(&self) -> u16 {
    self.rect.x
  }

  pub fn bottom(&self) -> u16 {
    self.rect.y + self.rect.height
  }

  pub fn right(&self) -> u16 {
    self.rect.x + self.rect.width
  }

  pub fn to_vec(&self) -> Vec<(u16, u16, &Cell)> {
    let mut result: Vec<(u16, u16, &Cell)> = vec![];

    for y in 0..self.height() {
      for x in 0..self.width() {
        result.push((x, y, self.get(x, y)));
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn buffer_index_of() {
    let buf = Buffer::filled(Rect::new(0, 0, 4, 3), Cell::default());
    let mut counter: usize = 0;

    for y in 0..buf.rect.height {
      for x in 0..buf.rect.width {
        let idx = buf.index_of(x, y);
        assert_eq!(idx, counter);
        counter += 1;
      }
    }
  }
}
