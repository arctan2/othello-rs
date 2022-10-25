use std::{rc::Rc, cell::{RefCell, RefMut, Ref}};

use super::{
  elements::Element,
  buffer::{Rect, Buffer}
};

pub struct Window {
  buffer: Buffer,
  sub_windows: Vec<WinRef>,
  parent: Option<WinRef>
}

type WinRef = Rc<RefCell<Window>>;

impl Window {
  pub fn default() -> Self {
    Window::new(0, 0, 0, 0)
  }

  pub fn new(width: u16, height: u16, x: u16, y: u16) -> Self {
    Window {
      buffer: Buffer::empty(Rect::new(x, y, width, height)),
      sub_windows: vec![],
      parent: None
    }
  }

  pub fn update_pos(&mut self, x: u16, y: u16) {
    self.buffer.set_pos(x, y);
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.buffer.set_pos(x, y);
    self
  }

  pub fn parent(mut self, parent: WinRef) -> Self {
    self.parent.replace(parent);
    self
  }

  pub fn size(mut self, width: u16, height: u16) -> Self {
    self.buffer = Buffer::empty(Rect::new(self.buffer.left(), self.buffer.top(), width, height));
    self 
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


pub struct WindowRef(pub WinRef);

impl WindowRef {
  pub fn default() -> Self {
    Self::from_window(Window::default())
  }

  pub fn new(width: u16, height: u16, x: u16, y: u16) -> Self {
    WindowRef(Rc::new(RefCell::new(Window {
      buffer: Buffer::empty(Rect::new(x, y, width, height)),
      sub_windows: vec![],
      parent: None
    })))
  }

  pub fn from_window(win: Window) -> Self {
    WindowRef(Rc::new(RefCell::new(win)))
  }

  pub fn new_child(&mut self, win: Window) -> Self {
    let child = WindowRef(
      Rc::new(
        RefCell::new(
          win.parent(self.0.clone())
        )
      )
    );

    self.0.borrow_mut().sub_windows.push(child.0.clone());
    child
  }

  pub fn inner_mut(&mut self) -> RefMut<'_, Window> {
    self.0.borrow_mut()
  }
  
  pub fn inner(&mut self) -> Ref<'_, Window> {
    self.0.borrow()
  }

  pub fn update_pos(&mut self, x: u16, y: u16) {
    self.inner_mut().update_pos(x, y);
  }

  pub fn draw_element(&mut self, el: &dyn Element) {
    self.inner_mut().draw_element(el);
  }
}

