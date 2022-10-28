use std::{fmt, rc::Rc, cell::{RefCell, RefMut, Ref}};

use super::{
  elements::Element,
  buffer::{Rect, Buffer}
};

pub struct Window {
  buffer: Buffer,
  sub_windows: Vec<WindowRef>,
  parent: Option<WindowRef>
}

type WinRef = Rc<RefCell<Window>>;

impl fmt::Debug for Window {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Window {{\n  parent: {}, \n  buffer: {:#?}\n}}", match self.parent {
      None => "None",
      _ => "<WindowRef>"
    }, self.buffer())
  }
}

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

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.buffer.set_pos(x, y);
  }

  pub fn position(mut self, x: u16, y: u16) -> Self {
    self.buffer.set_pos(x, y);
    self
  }

  pub fn parent(mut self, parent: WindowRef) -> Self {
    self.parent.replace(parent);
    self
  }

  pub fn get_parent(&self) -> Option<WindowRef> {
    self.parent.clone()
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

  pub fn clear(&mut self) {
    self.buffer.reset();
  }

  pub fn children(&self) -> &Vec<WindowRef> {
    &self.sub_windows
  }

  pub fn abs_pos(&self) -> (u16, u16) {
    let mut top = self.buffer.top();
    let mut left = self.buffer.left();

    let mut parent = self.get_parent();

    loop {
      match parent {
        Some(p) => {
          top += p.top();
          left += p.left();
          parent = p.parent();
        },
        None => { return (top, left) }
      }
    }
  }
}


#[derive(Clone, Debug)]
pub struct WindowRef(WinRef);

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
          win.parent(self.clone())
        )
      )
    );

    self.0.borrow_mut().sub_windows.push(child.clone());
    child
  }

  pub fn inner_mut(&mut self) -> RefMut<'_, Window> {
    self.0.borrow_mut()
  }
  
  pub fn inner(&self) -> Ref<'_, Window> {
    self.0.borrow()
  }

  pub fn set_pos(&mut self, x: u16, y: u16) {
    self.inner_mut().set_pos(x, y);
  }

  pub fn draw_element(&mut self, el: &dyn Element) {
    self.inner_mut().draw_element(el);
  }

  pub fn clear(&mut self) {
    self.inner_mut().clear();
  } 

  pub fn get_width(&self) -> u16 {
    self.inner().buffer.width()
  }

  pub fn get_height(&self) -> u16 {
    self.inner().buffer.height()
  }

  pub fn top(&self) -> u16 {
    self.inner().buffer.top()
  }
  
  pub fn left(&self) -> u16 {
    self.inner().buffer.left()
  }

  pub fn bottom(&self) -> u16 {
    self.inner().buffer.bottom()
  }

  pub fn right(&self) -> u16 {
    self.inner().buffer.right()
  }

  pub fn parent(&self) -> Option<WindowRef> {
    self.inner().get_parent()
  }

  pub fn abs_pos(&self) -> (u16, u16) {
    self.inner().abs_pos()
  }
}
