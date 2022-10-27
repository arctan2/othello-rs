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

    let (maxx, maxy) = (root_buf.right(), root_buf.bottom());
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
        let a = root_buf.get_mut(x + left, y + top);
        let b = buf.get(x, y);
        a.bg = b.bg;
        a.fg = b.fg;
        a.style = b.style;
        a.symbol = b.symbol.clone();
      }
    }
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
