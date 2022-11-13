mod termin;
mod game;
mod menu;

use std::collections::HashMap;
use std::iter::Map;
use std::{time::Duration, io::Write};
use std::thread;
use std::io::stdout;
use crossterm::style::Color;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor
};
use game::{Game, board};
use menu::{Menu, Return};
use termin::elements::Rectangle;
use termin::{
  crossterm_handler::CrosstermHandler, terminal_window::Terminal,
};

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn game_loop<W: Write>(terminal: &mut Terminal<W>, mut game: Game) {
  terminal.clear();
  game.init_board();
  game.render_board();
  terminal.refresh().unwrap();
  sleep(2000);
}

enum Ctx {
  Name(String)
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));

  let mut game_ctx = HashMap::new();

  game_ctx.insert("name", Ctx::Name("".to_string()));

  let mut menu_map = Menu::new("Main Menu")
                .sub_menu("Start",
                  Menu::new("start new game")
                  .action("offline", &|terminal| -> Return {
                    let game = Game::new(terminal.root.clone());
                    game_loop(terminal, game);
                    Return::ToRoot
                  })
                  .back("back")
                )
                .back("quit");
  
  menu_map.run(&mut terminal, &mut game_ctx);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
