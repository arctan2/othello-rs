mod termin;
mod game;
mod menu;

use std::{time::Duration, io::Write};
use std::thread;
use std::io::stdout;
use crossterm::style::Color;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor
};
use menu::{Menu, Return};
use termin::elements::Rectangle;
use termin::{
  crossterm_handler::CrosstermHandler, terminal_window::Terminal,
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn hello<W: Write>(terminal: &mut Terminal<W>) -> Return {
  terminal.clear();
  let boxi = Rectangle::default().size(10, 5).bg(Color::Red);
  terminal.root.draw_element(&boxi);
  terminal.refresh().unwrap();
  sleep(2000);
  Return::ToRoot
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let mut menu_map = Menu::new("Main Menu")
                .sub_menu("Start",
                  Menu::new("start new game")
                  .action("opt1", &hello)
                  .back("back")
                )
                .back("quit");

  menu_map.run(&mut terminal);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
