use std::{rc::Rc, cell::RefCell};

use super::{
  elements::Element,
  buffer::{Rect, Buffer}
};

type ParentInfo = Rect;

pub struct Window {
  width: u16,
  height: u16,
  x: u16,
  y: u16,
  buffer: Buffer,
  sub_windows: Vec<WinRef>,
  parent: Option<ParentInfo>
}

type WinRef = Rc<RefCell<Window>>;

impl Window {
  pub fn default() -> Self {
    Window::new(0, 0, 0, 0)
  }

  pub fn new(width: u16, height: u16, x: u16, y: u16) -> Self {
    Window {
      width,
      height,
      x,
      y, 
      buffer: Buffer::empty(Rect::new(x, y, width, height)),
      sub_windows: vec![],
      parent: None
    }
  }

  pub fn new_child(&mut self, win: Window) -> WinRef {
    let child = Rc::new(RefCell::new(win.parent(self.as_parent())));
    self.sub_windows.push(child.clone());
    child
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
    self.width = width;
    self.height = height;
    self.buffer = Buffer::empty(Rect::new(self.x, self.y, width, height));
    self 
  }

  pub fn as_parent(&self) -> ParentInfo {
    ParentInfo { x: self.x, y: self.y, width: self.width, height: self.height }
  }

  pub fn draw_element(&mut self, el: &dyn Element) {
    el.draw(&mut self.buffer);
  }

  pub fn buffer(&self) -> &Buffer {
    &self.buffer   
  }

  pub fn buffer_mut(&mut self) -> &mut Buffer {
    &mut self.buffer
  }
}
