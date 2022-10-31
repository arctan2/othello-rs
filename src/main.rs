mod termin;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use termin::{
  crossterm_handler::CrosstermHandler,
  elements::{Rectangle, Text, InputBox},
  window::Window,
};
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  event::{Event, KeyCode},
  execute, style::Color, cursor::{self, position}
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let mut win1 = terminal.root.new_child(Window::default().position(0, 0).size(50, 20));

  terminal.handle_input(|handler| {
    let placeholder = Text::default().text("nope: ");
    let mut input_box = InputBox::default().size(30, 3).position(placeholder.get_text().len() as u16, 0);
    win1.draw_element(&placeholder);
    let s = win1.read_string(&mut input_box, handler);
  });


  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
