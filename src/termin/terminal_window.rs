use std::{io::{self, Write}};

use super::{crossterm_handler::CrosstermHandler, window::{Window, WindowRef}};

pub struct Terminal <W: Write> {
  pub handler: CrosstermHandler<W>,
  pub root: WindowRef
}

impl <W: Write> Terminal<W> {
  pub fn new(root: Window, handler: CrosstermHandler<W>) -> Terminal<W> {
    Terminal { handler, root: WindowRef::from_window(root) }
  }

  pub fn render(&mut self, win: &Window) {
    let buf = win.buffer();
    let mut a = self.root.inner_mut();
    let root_buf = a.buffer_mut(); 
    let top = buf.top();
    let left = buf.left();

    for y in 0..buf.height() {
      for x in 0..buf.width() {
        let a = root_buf.get_mut(x + left, y + top);
        let b = buf.get(x, y);
        a.bg = b.bg;
        a.fg = b.fg;
        a.style = b.style;
        a.symbol = b.symbol.clone();
      }
    }

    self.handler.draw(root_buf.to_vec().into_iter()).unwrap();
  }

  pub fn clear(&mut self) {
    self.root.0.borrow_mut().buffer_mut().reset();
  }

  pub fn flush(&mut self) -> io::Result<()> {
    self.handler.flush()
  }
}
