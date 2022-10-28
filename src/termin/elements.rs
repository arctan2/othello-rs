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

pub struct Text {
  rect: Rect,
  bg: Color,
  fg: Color,
  text: String
}

impl Text {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Self { rect: Rect::new(x, y, width, height), bg: Color::Reset, fg: Color::Reset, text: String::from("") }
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

  pub fn text(mut self, text: &str) -> Self {
    self.text = text.to_string();
    self
  }

  pub fn bg(mut self, bg: Color) -> Self {
    self.bg = bg;
    self
  }

  pub fn fg(mut self, fg: Color) -> Self {
    self.fg = fg;
    self
  }

  pub fn set_bg(&mut self, bg: Color) {
    self.bg = bg;
  }

  pub fn set_fg(&mut self, fg: Color) {
    self.fg = fg;
  }

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.rect.x = x;
    self.rect.y = y;
  }

  pub fn set_text(&mut self, text: &str) {
    self.text = text.to_string();
  }

  pub fn set_size(&mut self, width: u16, height: u16) {
    self.rect.width = width;
    self.rect.height = height;
  }
}

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