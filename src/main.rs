mod termin;
mod game;
mod menu;

use std::{time::Duration, io::Write};
use std::thread;
use std::io::{stdout};
use crossterm::event::KeyCode;
use crossterm::style::{Color, Attribute};
use crossterm::{
  terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  execute, cursor
};
use game::{Game, board};
use menu::{Menu, Return};
use termin::elements::{Rectangle, Text};
use termin::window::{Window, Position::*, draw_elements};
use termin::{
  crossterm_handler::CrosstermHandler, terminal_window::Terminal,
};

use crate::game::offline::Offline;
use crate::termin::elements::InputBox;

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

fn change_name<W: Write>(terminal: &mut Terminal<W>, ctx: &mut Ctx) -> Return {
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
    new_name
  });

  ctx.name = name;
  Return::None
}

fn offline<W: Write>(terminal: &mut Terminal<W>, _: &mut Ctx, no_of_players: u8) -> Return {
  terminal.clear();
  let mut offline_win = terminal.root.new_child(
    Window::default().size(30, 10).bg(Color::Rgb { r: 0, g: 180, b: 0 }).position(2, 2)
  );
  let rect = offline_win.rect();
  let mut cur_side = 'w';
  let mut white = Text::default().text("white").position(rect, CenterB).attr(Attribute::Underlined).fg(Color::White);
  let mut black = Text::default().text("black").position(rect, CenterB).fg(Color::Black);

  black.set_xy_rel(black.width() as i16, 1);
  white.set_xy_rel(-(white.width() as i16), 1);

  draw_elements!(offline_win,
    Text::default()
      .text("Play Offline")
      .fg(Color::Black)
      .attr(Attribute::Bold)
      .position(rect, CenterH)
      .xy_rel(0, 1),
    Text::default()
      .text(if no_of_players == 1 { "Choose your side:" } else { "Choose Player 1 side: " })
      .xy_rel(2, 4)
      .fg(Color::Black),
    white, black
  );

  offline_win.render();
  terminal.refresh().unwrap();

  loop {
    let mut do_render = false;
    match terminal.getch() {
      KeyCode::Left | KeyCode::Right => {
        match cur_side {
          'w' => {
            black.set_attr(Attribute::Underlined);
            white.set_attr(Attribute::Reset);
            cur_side = 'b';
            do_render = true;
          },
          'b' => {
            white.set_attr(Attribute::Underlined);
            black.set_attr(Attribute::Reset);
            cur_side = 'w';
            do_render = true;
          },
          _ => ()
        }
      },
      KeyCode::Enter => break,
      KeyCode::Esc => {
        offline_win.delete();
        return Return::None
      }
      _ => ()
    }
    if do_render {
      draw_elements!(offline_win, white, black);
      terminal.handler.draw_window(&offline_win).unwrap();
      terminal.flush().unwrap();
    }
  }

  offline_win.delete();

  use game::offline::ParticipantType::{Player, Bot};

  if no_of_players == 2 {
    Offline{black: Player, white: Player}
  } else {
    if cur_side == 'w' {
      Offline { black: Bot, white: Player }
    } else {
      Offline { black: Player, white: Bot }
    }
  }.begin_game(terminal);

  Return::ToRoot
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
                offline(terminal, ctx, 1)
              })
              .action("2 player", &|terminal, ctx| -> Return {
                offline(terminal, ctx, 2)
              })
              .back("back")
            )
            .sub_menu("online", 
              Menu::<Ctx>::new("Online")
              .action("", &|terminal, ctx| -> Return {
                Return::All
              })
            )
            .back("back")
          )
          .action("change name", &change_name)
          .back("quit");
  
  menu_map.run(&mut terminal, &mut game_ctx);

  execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
  disable_raw_mode().unwrap();
}
