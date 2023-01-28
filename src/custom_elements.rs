use crossterm::style::Color;

use crate::termin::{window::Position, elements::{Text, Rectangle, Element}, buffer::Rect};

pub struct DialogBox {
  text_box: Text,
  rect: Rectangle
}

impl DialogBox {
  pub fn new(width: u16, height: u16) -> Self {
    Self { text_box: Text::default().size(width - 2, height - 2).xy(1, 1), rect: Rectangle::default().size(width, height) }
  }

  pub fn set_text(&mut self, text: &str) {
    self.text_box.set_text(text);
  }

  pub fn text(mut self, text: &str) -> Self {
    self.set_text(text);
    self
  }

  pub fn align_x(mut self) -> Self {
    self.text_box.set_position(self.rect.rect(), Position::CenterH);
    self
  }

  pub fn align_y(mut self) -> Self{
    self.text_box.set_position(self.rect.rect(), Position::CenterV);
    self
  }

  pub fn set_position(&mut self, rect: Rect, pos: Position) {
    self.rect.set_position(rect, pos);
    self.text_box.set_position(self.rect.rect(), Position::Coord(1, 1));
  }

  pub fn position(mut self, rect: Rect, pos: Position) -> Self {
    self.set_position(rect, pos);
    self
  }

  pub fn set_bg(&mut self, bg: Color) {
    self.rect.set_bg(bg);
  }

  pub fn bg(mut self, bg: Color) -> Self {
    self.set_bg(bg);
    self
  }

  pub fn xy_rel(&mut self, dx: i16, dy: i16) {
    self.text_box.set_xy_rel(dx, dy);
  }

  pub fn error(&mut self, text: &str) {
    self.rect.set_bg(Color::Red);
    self.set_text(text);
  }

  pub fn info(&mut self, info: &str) {
    self.rect.set_bg(Color::Blue);
    self.set_text(info);
  }
}

impl Element for DialogBox {
  fn draw(&self, buf: &mut crate::termin::buffer::Buffer) {
    self.rect.draw(buf);
    self.text_box.draw(buf);
  }
}
