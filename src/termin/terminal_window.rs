use std::io::{self, Write};

use super::{crossterm_handler::CrosstermHandler, window::{Window, WindowRef}};

pub struct Terminal <W: Write> {
  pub handler: CrosstermHandler<W>,
  pub root: WindowRef
}

impl <W: Write> Terminal<W> {
  pub fn new(root: Window, handler: CrosstermHandler<W>) -> Terminal<W> {
    Terminal { handler, root: WindowRef::from_window(root) }
  }

  pub fn clear(&mut self) {
    self.root.inner_mut().buffer_mut().reset();
  }

  pub fn flush(&mut self) -> io::Result<()> {
    match self.handler.draw(self.root.inner().buffer().to_vec().into_iter()) {
      Ok(()) => (),
      Err(_) => panic!("error while drawing the buffer")
    }
    self.handler.flush()
  }
}

/*
This line must be above the self.inner_mut() as parent contains the win
and will error that "already mutably borrowed: BorrowError"
because we can't have immutable(win) varible be used after mutable(root)

here's the structure

parent: {
  sub_windows: [win]
}

parent: {
^^^^^ we are taking this as mutable
  sub_windows: [win]
                ^^^ but this is immut and can't be used after parent is borrowed "mutably"
}

And why does the Rc-RefCell deep check stuff??
Like how does it know that win is inside root??

I hope future me won't waste time on this again.
*/