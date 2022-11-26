mod termin;
mod game;
mod menu;

use std::{time::Duration, io::Write};
use std::thread;
use std::io::{stdout};
use crossterm::style::Color;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor
};
use game::{Game, board};
use menu::{Menu, Return};
use termin::elements::{Rectangle, Text};
use termin::window::{Window, Position::Center};
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

struct Ctx {
  name: String
}

fn change_name<W: Write>(terminal: &mut Terminal<W>, ctx: &mut Ctx) -> Return {
  terminal.root.clear();
  terminal.refresh().unwrap();

  let name = terminal.handle_input(|handler, root| -> String {
    let mut input_win = root.new_child(Window::default().size(50, 10).bg(Color::Red));
    let mut heading = Text::default().text("Change Name");
    let label = Text::default().text("new name: ").position(0, 2);
    let mut input = InputBox::default()
                    .max_len(30)
                    .position(label.get_rect().x + label.get_rect().width, label.get_rect().y)
                    .size(20, 2).start_text((0, 0));
    
    heading.set_position(input_win.rect(), Center{h: true, v: false});
    
    input_win.set_position(Center{h: true, v: true});
    input_win.draw_element(&label);
    input_win.draw_element(&heading);
    input_win.render();

    handler.draw_window(&root).unwrap();

    let new_name = input_win.read_string(&mut input, handler);
    new_name
  });

  ctx.name = name;
  Return::None
}

fn main() {
  enable_raw_mode().unwrap();
  execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

  let mut terminal = termin::root(CrosstermHandler::new(stdout()));

  let mut game_ctx = Ctx { name: "Player".to_string() };

  let mut menu_map = Menu::<Ctx>::new("Main Menu")
          .routine(&|menu, ctx| {
            let mut s = "Welome ".to_string();
            s.push_str(&ctx.name);
            menu.heading.set_text(&s);
            menu.heading.width_fit();
          })
          .sub_menu("play game",
            Menu::<Ctx>::new("Play game")
            .action("offline", &|terminal, _| -> Return {
              let game = Game::new(terminal.root.clone());
              game_loop(terminal, game);
              Return::ToRoot
            })
            .back("back")
          )
          .action("change name", &change_name)
          .back("quit");
  
  menu_map.run(&mut terminal, &mut game_ctx);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
