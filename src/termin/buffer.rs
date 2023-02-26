use std::{fmt, vec};

use crossterm::style::{Color, Attribute};

use crate::sleep;

#[derive(Clone, Debug)]
pub struct Cell {
  pub bg: Color,
  pub fg: Color,
  pub attr: Attribute,
  pub symbol: String
}

impl Default for Cell {
  fn default() -> Self {
    Cell { bg: Color::Reset, fg: Color::Reset, attr: Attribute::Reset, symbol: " ".into() }
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

  pub fn set_attr(&mut self, attr: Attribute) {
    self.attr = attr;
  }

  pub fn reset(&mut self) {
    self.bg = Color::Reset;
    self.fg = Color::Reset;
    self.symbol = String::from(" ");
    self.attr = Attribute::Reset;
  }
}

#[derive(Debug, Clone, Copy)]
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

  pub fn get_xy(&self) -> (u16, u16) {
    (self.x, self.y)
  }

  pub fn get_center_start_pos(&self, rect: Rect) -> (u16, u16) {
    let h = (self.width / 2) - (rect.width / 2);
    let v = (self.height / 2) - (rect.height / 2);
    (h, v)
  }
}

impl Default for Rect {
  fn default() -> Self {
    Self::new(0, 0, 0, 0) 
  }
}

pub struct Buffer {
  bg: Color,
  rect: Rect,
  scroll: Rect,
  content: Vec<Cell>
}

impl fmt::Debug for Buffer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Buffer{{rect: {:?}, content: Vec<Cell, {}>}}", self.rect, self.content.len())
  }
}

impl Buffer {
  pub fn empty(rect: Rect, scroll: Rect) -> Self {
    let a = scroll.area() as usize;
    Buffer{ rect, scroll, content: vec![Cell::default(); a], bg: Color::Reset }
  }
  
  pub fn filled(rect: Rect, scroll: Rect, fill: Cell) -> Buffer {
    let a = scroll.area() as usize;
    Buffer { rect, scroll, content: vec![fill.clone(); a], bg: fill.bg }
  }

  fn resize_content(&mut self) {
    let a = self.scroll.area() as usize;
    let mut s = self.content[0].clone();
    s.bg = self.bg;
    s.symbol = " ".to_string();
    self.content = vec![s; a];
  }

  pub fn set_scroll_size(&mut self, width: u16, height: u16) {
    self.scroll.width = width;
    self.scroll.height = height;
    self.resize_content();
  }

  pub fn set_scroll_xy(&mut self, x: u16, y: u16) {
    self.scroll.x = x;
    self.scroll.y = y;
  }

  pub fn content_mut(&mut self) -> &mut Vec<Cell> {
    &mut self.content
  }
  
  pub fn content(&self) -> &Vec<Cell> {
    &self.content
  }

  pub fn reset(&mut self) {
    for c in &mut self.content {
      c.reset();
      c.set_bg(self.bg);
    }
  }

  pub fn width(&self) -> u16 {
    self.rect.width
  }

  pub fn height(&self) -> u16 {
    self.rect.height
  }

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.rect.x = x;
    self.rect.y = y;
  }

  pub fn index_of(&self, mut x: u16, mut y: u16) -> usize {
    y += self.scroll.y;
    x += self.scroll.x;
    ((self.rect.width * y) + x) as usize
  }

  pub fn size(&self) -> usize {
    self.content.len()
  }

  pub fn get(&self, x: u16, y: u16) -> &Cell {
    let idx = self.index_of(x, y);
    &self.content[idx]
  }

  pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
    let idx = self.index_of(x, y);
    &mut self.content[idx]
  }

  pub fn set_bg(&mut self, bg: Color) {
    self.bg = bg;
    for c in &mut self.content {
      c.set_bg(bg);
    }
  }

  pub fn get_bg(&self) -> Color {
    self.bg
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

  pub fn rect(&self) -> Rect {
    self.rect
  }

  pub fn scroll(&self) -> Rect {
    self.scroll
  }

  pub fn to_vec(&self, abs: (u16, u16)) -> Vec<(u16, u16, &Cell)> {
    let mut result: Vec<(u16, u16, &Cell)> = vec![];

    for y in 0..self.height() {
      for x in 0..self.width() {
        result.push((x + abs.1, y + abs.0, self.get(x, y)));
      }
    }
    result
  }
}
