mod termin;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::Rectangle,
  window::Window,
};
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, style::Color, cursor
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let mut win1 = terminal.root.new_child(Window::default().size(10, 5).position(0, 0));
  let el1 = Rectangle::default()
              .size(win1.get_width(), win1.get_height())
              .position(0, 0)
              .bg(Color::Blue);

  for i in 0..180 {
    win1.set_pos(i, 0);
    win1.draw_element(&el1);
    terminal.clear();
    terminal.render(&win1);
    terminal.flush().unwrap();
    sleep(1);
  }

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
