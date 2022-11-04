mod termin;
mod game;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor, style::Color
};
use game::Game;
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

  let mut game = Game::new(terminal.root.clone());

  game.init_board();
  game.board.render();
  terminal.refresh().unwrap();

  for y in 0..8 {
    for x in 0..8 {
      terminal.clear();
      game.board.move_cursor(x, y);
      game.board.render();
      terminal.refresh().unwrap();
      sleep(1000);
    } 
  }

  sleep(5000);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
