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
  let (width, height) = (32 - 2, 15);
  let mut board_container = terminal.root.new_child(
    Window::default().size(width + 4, height + 2).bg(Color::Green)
  );
  let mut board = board_container.new_child(Window::default().size(32 - 2, 15).position(2, 1));

  let mut game = Game::default();

  game.init_board();

  game.board.print_board(board.clone());

  board.render_to_parent();

  terminal.draw_window(&board_container);

  terminal.flush().unwrap();

  sleep(5000);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
