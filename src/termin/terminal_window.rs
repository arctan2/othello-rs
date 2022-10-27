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

  pub fn render(&mut self, win: &WindowRef) {
    let win = win.inner();
    let buf = win.buffer();

    /*
    This line must be above the self.root.inner_mut() as root contains the win
    and will error that "already mutably borrowed: BorrowError"
    because we can't have immutable(win) varible be used after mutable(root)

    here's the structure

    Terminal {
      root: {
        sub_windows: [win]
      }
    }

    Terminal {
      root: {
      ^^^^^ we are taking this as mutable
        sub_windows: [win]
                      ^^^ but this is immut and can't be used after root is borrowed "mutably"
      }
    }

    And why does the Rc-RefCell deep check stuff??
    Like how does it know that win is inside root??
    
    I hope future me won't waste time on this again.
    */
    let (top, left) = win.abs_pos();
    let mut a = self.root.inner_mut();
    let root_buf = a.buffer_mut(); 

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
    self.root.inner_mut().buffer_mut().reset();
  }

  pub fn flush(&mut self) -> io::Result<()> {
    self.handler.flush()
  }
}
