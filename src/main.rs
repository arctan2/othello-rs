mod termin;

use std::time::Duration;
use std::thread::sleep;
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

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let mut win1 = terminal.root.new_child(Window::default().size(60, 10).position(0, 0));
  let mut el1 = Rectangle::default().size(2, 1).position(0, 0).bg(Color::Blue);
  let mut colo = Color::Red;

  for y in 0..10 {
    for x in 0..10 {
      el1.set_bg(colo);
      win1.draw_element(&el1);
      terminal.render(&win1.inner());
      terminal.flush().unwrap();
      sleep(Duration::from_millis(100));
      el1.set_pos(x, y);

      colo = if colo == Color::Red {
        Color::Blue
      } else {
        Color:: Red
      };
      
      win1.clear();
    }
  }

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
