mod termin;

use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::Rectangle,
  window::Window
};
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, style::Color
};

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let win1 = terminal.root.new_child(Window::default().size(5, 5).position(0, 0));
  let el1 = Rectangle::default().size(0, 0).position(0, 0).bg(Color::Blue);

  win1.borrow_mut().draw_element(&el1);
  terminal.render(&win1.borrow());
  terminal.flush().unwrap();

  sleep(Duration::from_millis(10000));

  execute!(stdout(), LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
