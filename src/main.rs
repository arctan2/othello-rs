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
use game::{Game, board};
use menu::{Menu, Return};
use termin::elements::{Rectangle, Text};
use termin::{
  crossterm_handler::CrosstermHandler, terminal_window::Terminal,
};

use crate::termin::elements::InputBox;

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

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));

  struct Ctx {
    name: String
  }

  let mut game_ctx = Ctx { name: "Player".to_string() };

  let mut menu_map = Menu::<Ctx>::new("Main Menu")
          .routine(&|menu, ctx| {
            let mut s = "Welome ".to_owned();
            s.push_str(&ctx.name);
            menu.heading.set_text(&s);
            menu.heading.width_fit();
          })
          .sub_menu("create game",
            Menu::<Ctx>::new("Create game")
            .action("offline", &|terminal, _| -> Return {
              let game = Game::new(terminal.root.clone());
              game_loop(terminal, game);
              Return::ToRoot
            })
            .back("back")
          )
          .action("Change name", &|terminal, mut ctx| -> Return {
            terminal.root.clear();

            let name = terminal.handle_input(|handler, root| -> String {
              let heading = Text::default().text("Change Name");
              let label = Text::default().text("new name: ").position(0, 2);
              let mut input = InputBox::default()
                              .position(label.get_rect().x, label.get_rect().y)
                              .size(20, 1).start_text((label.get_text().len() as u16, 0));
              root.draw_element(&label);
              root.draw_element(&heading);
              root.read_string(&mut input, handler)
            });

            ctx.name = name;
            Return::None
          })
          .back("quit");
  
  menu_map.run(&mut terminal, &mut game_ctx);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
