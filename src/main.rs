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
  let mut win1 = terminal.root.new_child(Window::default().size(2, 1).position(0, 0));
  let el1 = Rectangle::default().size(0, 0).position(0, 0).bg(Color::Blue);

  for i in 1..=10 {
    for j in 1..50 {
      win1.draw_element(&el1);
      terminal.render(&win1.inner());
      terminal.flush().unwrap();
      sleep(Duration::from_millis(10));
      win1.update_pos(j, i);
      terminal.clear();
    }
  }

  execute!(stdout(), LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
