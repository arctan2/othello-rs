use crossterm::style::Color;

use crate::termin::buffer::Rect;

use super::impl_setters;

pub struct InputBox {
  rect: Rect,
  placeholder: String,
  fg: Color,
  bg: Color
}

impl Default for InputBox {
  fn default() -> Self {
    Self { rect: Rect::default(), placeholder: String::new(), fg: Color::Reset, bg: Color::Reset }
  }
}

impl InputBox {
  pub fn new(placeholder: &str) -> Self {
    Self::default().placeholder(placeholder)
  }

  pub fn placeholder(mut self, placeholder: &str) -> Self {
    self.placeholder = placeholder.to_string();
    self
  }
}

impl_setters!(InputBox);