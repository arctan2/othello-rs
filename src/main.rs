mod termin;

use std::{io::{stdout}};
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::{Rectangle},
  window::{Window, push_windows}
};

fn main() {
  let handler = CrosstermHandler::new(stdout());

  let mut root_win = termin::root(&handler);

  let mut win1 = Window::default(&handler).size(5, 5).position(0, 0);

  push_windows!(root_win, win1);

  win1.render();
  
  let mut el1 = Rectangle::default().size(0, 0).position(0, 0);

  el1 = el1.position(0, 0);

  win1.draw_element(&el1);
  win1.render_element(&el1);
  win1.render();
}
