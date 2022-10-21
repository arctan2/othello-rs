use std::{io::{self, Write}};

use super::{crossterm_handler::CrosstermHandler, window::Window};

pub struct Terminal <W: Write> {
  pub handler: CrosstermHandler<W>,
  pub root: Window
}

impl <W: Write> Terminal<W> {
  pub fn new(root: Window, handler: CrosstermHandler<W>) -> Terminal<W> {
    Terminal { handler, root }
  }

  pub fn render(&mut self, win: &Window) {
    let buf = win.buffer();
    let root_buf = self.root.buffer_mut(); 
    println!("{:?}", buf);

    for y in buf.top()..buf.bottom() {
      for x in buf.left()..buf.right() {
        println!("({}, {})", x, y);
        let a = root_buf.get_mut(x, y);
        let b = buf.get(x, y);
        a.bg = b.bg;
        a.fg = b.fg;
        a.style = b.style;
        a.symbol = b.symbol.clone();
      }
    }

    self.handler.draw(root_buf.to_iter().into_iter()).unwrap();
  }

  pub fn flush(&mut self) -> io::Result<()> {
    self.handler.flush()
  }
}
