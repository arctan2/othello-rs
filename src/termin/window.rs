use std::io::Write;

use crate::termin::CrosstermHandler;

use super::elements::Element;

pub struct Window<'a, W: Write> {
  start: (u16, u16),
  width: u16,
  height: u16,
  handler: &'a CrosstermHandler<W>,
  sub_windows: Vec<Box<&'a Window<'a, W>>>
}

#[macro_export]
macro_rules! push_windows {
  ($parent: ident, $($child:expr),*) => {
    $(
      $parent.push_window(&$child)
    )*
  };
}

impl <'a, W: Write> Window<'a, W> {
  pub fn default(handler: &'a CrosstermHandler<W>) -> Self {
    Window { width: 0, height: 0, start: (0, 0), handler: handler, sub_windows: vec![] }
  }

  pub fn new(handler: &'a CrosstermHandler<W>, width: u16, height: u16, start: (u16, u16)) -> Self {
    Window { width, height, start, handler, sub_windows: vec![] }
  }

  pub fn position(mut self, pos: (u16, u16)) -> Self {
    self.start = pos;
    self
  }

  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.width = width;
    self.height = height;
    self 
  }

  pub fn push_window(mut self, win: &'a Self) {
    self.sub_windows.push(Box::new(win));
  }

  pub fn render(&self) {
  }

  pub fn render_element(&self, el: &dyn Element) {
  }

  pub fn draw_element(&self, el: &dyn Element) {
  }
}
