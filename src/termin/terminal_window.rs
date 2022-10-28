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

  pub fn render_window(&mut self, win: &WindowRef) {
    self.root.render_window(win);
  }

  // can be optimised
  pub fn render_children(&mut self, win: &WindowRef) {
    let w = win.inner();
    let children = w.children();
    
    for child in children {
      self.render_window(child);
    }
  }

  // can be optimised
  pub fn render_all(&mut self, win: &WindowRef) {
    self.render_window(win);
    self.render_children(win);
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

macro_rules! render_windows {
  ($terminal:ident, $($win:ident),+) => {
    $($terminal.render(&$win);)+
  }
}

pub(crate) use render_windows;
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