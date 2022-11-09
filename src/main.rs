mod termin;
mod game;
mod menu;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor, style::Color
};
use game::Game;
use menu::Menu;
use termin::{
  crossterm_handler::CrosstermHandler,
  window::Window, elements::Rectangle
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), cursor::Hide, EnterAlternateScreen).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let menu_map = Menu::new("Main Menu")
                .sub_menu("start",
                  Menu::new("start new game")
                  .action("opt0", &|| {})
                  .action("opt1", &|| {})
                  .back("back")
                )
                .back("quit");

  menu_map.run(&mut terminal);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
