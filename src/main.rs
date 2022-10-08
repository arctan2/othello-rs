mod termin;

use std::{io::{stdout}};
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::{Rectangle},
  window::{Window}
};

fn main() {
  let handler = CrosstermHandler::new(stdout());

  let mut root_win = termin::Root(&handler);

  let win1 = Window::default(&handler).size(20, 20);

  push_windows!(root_win, win1);

  win1.render();
  
  let el1 = Rectangle::default().size(0, 0).position(0, 0);

  win1.draw_element(&el1);
  win1.render_element(&el1);
  win1.render();
}
