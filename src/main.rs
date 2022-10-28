mod termin;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::{Rectangle, Text},
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
  let mut win1 = terminal.root.new_child(Window::default().size(20, 10).position(5, 5));
  let mut win2 = win1.new_child(Window::default().size(10, 5).position(1, 1));
  let text = Text::default().size(10, 5).text("hehe noi").fg(Color::Red).bg(Color::DarkBlue);
  let rect = Rectangle::default().size(win1.get_width(), win1.get_height()).bg(Color::Red);

  win1.draw_element(&rect);
  win2.draw_element(&text);

  terminal.render_all(&win1);

  terminal.flush().unwrap();

  sleep(2000);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
