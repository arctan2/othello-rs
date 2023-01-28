mod termin;
mod game;
mod menu;
mod custom_elements;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor, style::Color
};
use custom_elements::DialogBox;
use game::macros::choose_side_win;
use game::online::Online;
use menu::{Menu, Return};
use termin::elements::Text;
use termin::terminal_window::TerminalHandler;
use termin::window::{Window, Position::*};
use termin::{
  crossterm_handler::CrosstermHandler
};

use crate::game::offline::play_offline;
use crate::termin::elements::InputBox;

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn change_name(terminal: &mut TerminalHandler, ctx: &mut Ctx) -> Return {
  terminal.root.clear();
  terminal.refresh().unwrap();

  let name = terminal.handle_input(|handler, root| -> String {
    let mut input_win = root.new_child(Window::default().size(50, 10));
    let label = Text::default().text("new name: ").xy(0, 2);
    let mut input = InputBox::default()
                    .max_len(20)
                    .position(label.x() + label.width(), label.y())
                    .size(21, 1).start_text((0, 0));
    

    input_win.set_xy_rel(2, 2);
    input_win.draw_element(&label);
    input_win.draw_text("Change Name", CenterH);
    input_win.render();

    handler.draw_window(&root).unwrap();

    let new_name = input_win.read_string(&mut input, handler);
    input_win.delete();
    new_name
  });

  ctx.name = name;
  Return::None
}


struct Ctx {
  name: String
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
            .sub_menu("offline", 
              Menu::<Ctx>::new("Offline")
              .action("1 player", &|terminal, ctx| -> Return {
                play_offline(terminal, ctx, 1)
              })
              .action("2 player", &|terminal, ctx| -> Return {
                play_offline(terminal, ctx, 2)
              })
              .back("back")
            )
            .sub_menu("online", 
              Menu::<Ctx>::new("Online")
              .action("create game", &|terminal, ctx| -> Return {
                let host_side = choose_side_win!(terminal, "Create Game Online", "Choose your side:");
                Online::default()
                  .set_player(host_side, &ctx.name)
                  .create_and_start(terminal);
                Return::None
              })
              .action("join game", &|terminal, ctx| -> Return {
                Return::All
              })
              .back("back")
            )
            .back("back")
          )
          .action("change name", &change_name)
          .back("quit");
  
  menu_map.run(&mut terminal, &mut game_ctx);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
