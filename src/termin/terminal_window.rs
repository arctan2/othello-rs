use std::io::{self, Write};
use crossterm::event::{Event, KeyEvent, KeyCode};

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

  pub fn event(&self) -> Event {
    self.handler.event()
  }

  pub fn getch(&self) -> KeyCode {
    loop {
      match self.event() {
        Event::Key(k) => return k.code,
        _ => ()
      }
    }
  }

  pub fn refresh(&mut self) -> io::Result<()> {
    match self.handler.draw(self.root.inner().buffer().to_vec((0, 0)).into_iter()) {
      Ok(()) => (),
      Err(_) => panic!("error while drawing the buffer")
    }
    self.flush()
  }

  pub fn draw_window(&mut self, win: &WindowRef) {
    match self.handler.draw(win.inner().buffer().to_vec((0, 0)).into_iter()) {
      Ok(()) => (),
      Err(_) => panic!("error while drawing the buffer")
    }
  }

  pub fn handle_input<F, R>(&mut self, func: F) -> R
  where
    F: FnOnce(&mut CrosstermHandler<W>, &mut WindowRef) -> R
  {
    func(&mut self.handler, &mut self.root)
  }

  pub fn handle_input_ctx<F, T, R>(&mut self, func: F, ctx: T) -> R
  where
    F: FnOnce(&mut CrosstermHandler<W>, &mut WindowRef, T) -> R
  {
    func(&mut self.handler, &mut self.root, ctx)
  }

  pub fn flush(&mut self) -> io::Result<()> {
    self.handler.flush()
  }
}