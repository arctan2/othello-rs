use std::{fmt, rc::Rc, cell::{RefCell, RefMut, Ref}, io::Write};

use crossterm::style::Color;

use super::{
  elements::{Element, InputBox},
  buffer::{Rect, Buffer}, crossterm_handler::CrosstermHandler
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

  pub fn bg(mut self, bg: Color) -> Self {
    self.buffer.set_bg(bg);
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

  pub fn set_bg(&mut self, bg: Color) {
    self.buffer.set_bg(bg);
  }

  pub fn clear(&mut self) {
    self.buffer.reset();
  }

  pub fn children(&self) -> Vec<WindowRef> {
    self.sub_windows.clone()
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

  pub fn rel_pos(&self) -> (u16, u16) {
    (self.buffer.top(), self.buffer.left())
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

  pub fn set_bg(&mut self, bg: Color) {
    self.inner_mut().set_bg(bg);
  }

  fn render_window_at(&mut self, buf: &Buffer, top: u16, left: u16) {
    let mut a = self.inner_mut();
    let self_buf_mut = a.buffer_mut();

    let (maxx, maxy) = (self_buf_mut.right(), self_buf_mut.bottom());
    let mut endx = buf.width() as i16;
    let mut endy = buf.height() as i16;
    
    if left + buf.width() > maxx {
      endx -= ((left + buf.width()) - maxx) as i16;
    }
    if top + buf.height() > maxy {
      endy -= ((top + buf.height()) - maxy) as i16;
    }

    if endx < 0 || endy < 0 {
      return;
    }

    for y in 0..(endy as u16) {
      for x in 0..(endx as u16) {
        let a = self_buf_mut.get_mut(x + left, y + top);
        let b = buf.get(x, y);
        if b.bg != Color::Reset {
          a.bg = b.bg;
        }
        a.fg = b.fg;
        a.style = b.style;
        if b.symbol != " " {
          a.symbol = b.symbol.clone();
        } 
      }
    }
  }

  pub fn render_window(&mut self, win: &WindowRef) {
    let win = win.inner();
    let buf = win.buffer();

    let (top, left) = win.rel_pos();
    self.render_window_at(buf, top, left);
  }

  pub fn render_to_parent(&mut self) {
    match self.parent() {
      Some(mut parent) => parent.render_window(&self),
      None => panic!("cannot call window.render_to_parent() on root window.")
    }
  }

  pub fn render(&mut self) {
    let mut parent = self.parent();
    let w = self.inner();
    let buf = w.buffer();
    let mut top = buf.top();
    let mut left = buf.left();

    loop {
      match parent {
        Some(mut p) => {
          parent = p.parent();
          if parent.is_none() {
            p.render_window_at(&buf, top, left);
            return;
          }
          top += p.top();
          left += p.left();
        },
        None => panic!("cannot call window.render() on root window, instead call terminal.flush().")
      }
    }
  }

  pub fn is_root(&self) -> bool {
    match self.parent() {
      Some(_) => false,
      None => true
    }
  }

  // can be optimized
  pub fn render_children(&mut self) {
    let children = self.inner_mut().children();

    if !self.is_root() {
      self.render();
    }

    for mut child in children {
      child.render();
    }
  }

  // can be optimized
  pub fn render_deep(&mut self) {
    let children = self.inner_mut().children();

    if !self.is_root() {
      self.render();
    }

    for mut child in children {
      child.render();
      child.render_deep();
    }
  }

  pub fn render_element(&mut self, el: &dyn Element) {
    self.draw_element(el);
    self.render();
  }

  pub fn read_string<W: Write>(&mut self, input_box: &mut InputBox, handler: &mut CrosstermHandler<W>) -> String {
    input_box.read_string(self, handler)
  }
}
