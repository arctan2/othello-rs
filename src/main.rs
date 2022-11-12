mod termin;
mod game;
mod menu;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor
};
use menu::{Menu, Return};
use termin::{
  crossterm_handler::CrosstermHandler,
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn t() -> Return {
  return Return::All
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));
  let mut menu_map = Menu::new("Main Menu")
                .sub_menu("Start",
                  Menu::new("start new game")
                  .sub_menu("opt0",
                    Menu::new("hehehehaw")
                    .action("all the way back", &t)
                    .back("bakc")
                  )
                  .action("opt1", &|| -> Return { Return::None })
                  .back("back")
                )
                .back("quit");

  menu_map.run(&mut terminal);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
