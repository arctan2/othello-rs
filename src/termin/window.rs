use std::{io::Write};
use super::{
  elements::Element,
  crossterm_handler::CrosstermHandler,
  buffer::{Cell, Rect}
};

type ParentInfo = Rect;

pub struct Window<'a, W: Write> {
  width: u16,
  height: u16,
  x: u16,
  y: u16,
  handler: &'a CrosstermHandler<W>,
  buffer: Vec<Cell>,
  sub_windows: Vec<WinRef<'a, W>>,
  parent: Option<ParentInfo>
}

type WinRef<'a, W> = Box<&'a Window<'a, W>>; 

macro_rules! push_windows {
  ($parent: ident, $($child:expr),*) => {
    $(
      $child = $child.parent($parent.as_parent());
      $parent.push_window(&$child);
    )*
  };
}

pub(crate) use push_windows;

impl <'a, W: Write> Window<'a, W> {
  pub fn default(handler: &'a CrosstermHandler<W>) -> Self {
    Window::new(handler, 0, 0, 0, 0)
  }

  pub fn new(handler: &'a CrosstermHandler<W>, width: u16, height: u16, x: u16, y: u16) -> Self {
    let buf: Vec<Cell> = vec!(Cell::default(); (width * height) as usize);

    Window {
      width,
      height,
      x,
      y, 
      handler, 
      buffer: buf,
      sub_windows: vec![],
      parent: None
    }
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.x = x;
    self.y = y;
    self
  }

  pub fn parent(mut self, parent: ParentInfo) -> Self {
    self.parent.replace(parent);
    self
  }

  pub fn size(mut self, width: u16, height: u16) -> Self {
    let buf: Vec<Cell> = vec!(Cell::default(); (width * height) as usize);
    self.width = width;
    self.height = height;
    self.buffer = buf;
    self 
  }

  pub fn as_parent(&self) -> ParentInfo {
    ParentInfo { x: self.x, y: self.y, width: self.width, height: self.height }
  }

  pub fn push_window(&mut self, win: &'a Self) {
    self.sub_windows.push(Box::new(win));
  }

  pub fn render(&self) {
  }

  pub fn render_element(&mut self, el: &dyn Element) {
  }

  pub fn draw_element(&mut self, el: &dyn Element) {
  }
}
