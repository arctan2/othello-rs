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
use menu::{Menu, Return, MenuRoot};
use termin::elements::{Rectangle, Text};
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
  Name(&'static str)
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));

  let mut game_ctx = HashMap::new();

  game_ctx.insert("name", Ctx::Name(""));

  let mut menu_map = MenuRoot::new(Menu::new("Main Menu")
                .sub_menu("Start",
                  Menu::<Ctx>::new("start new game")
                  .action("offline", &|terminal, _| -> Return {
                    let game = Game::new(terminal.root.clone());
                    game_loop(terminal, game);
                    Return::ToRoot
                  })
                  .action("change name", &|terminal, ctx| -> Return {
                    ctx.insert("name", Ctx::Name("hehehehaw huahahahha"));
                    let t = Text::default().text(match ctx.get("name").unwrap() {
                      Ctx::Name(n) => n
                    });
                    terminal.root.clear();
                    terminal.root.draw_element(&t);
                    terminal.refresh().unwrap();
                    sleep(2000);
                    Return::ToRoot
                  })
                  .back("back")
                )
                .back("quit"), game_ctx);
  
  menu_map.run(&mut terminal);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
