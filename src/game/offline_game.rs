use rand::Rng;

use crate::{ 
  termin::{
    terminal_window::TerminalHandler,
    window::{Window, draw_elements}, 
  },
  sleep,
  menu::Return, game::macros::choose_side_win
};

use super::{Game, board::WHITE};

#[derive(Copy, Clone)]
pub enum ParticipantType {
  Bot,
  Player
}

pub struct Offline {
  pub black: ParticipantType,
  pub white: ParticipantType
}

fn rand_item_from_vec<T: Copy>(v: &Vec<T>) -> T {
  let mut rng = rand::thread_rng(); 
  v[rng.gen_range(0..v.len())]
}

impl Offline {
  pub fn begin_game(&self, terminal: &mut TerminalHandler) {
    terminal.clear();
    let mut offline_win = terminal.root.new_child(Window::default().size(terminal.root.width(), terminal.root.height()));
    let mut game = Game::new(offline_win.clone());
    let mut cur_turn = self.black;

    game.init_board();
    game.render_cur_turn_side();

    while !game.is_over {
      match cur_turn {
        ParticipantType::Player => {
          game.board.calc_available_moves(game.cur_turn_side);
          if !game.board.available_moves.is_empty() {
            game.board.place_cursor_on_legal_position();
            game.enable_cursor_movement(terminal);
          } else {
            sleep(1000);
          }
          
          game.toggle_side();
          cur_turn = if game.cur_turn_side == WHITE { self.white } else { self.black };
        },
        ParticipantType::Bot => {
          game.render_board();
          terminal.refresh().unwrap();

          game.board.calc_available_moves(game.cur_turn_side);

          if !game.board.available_moves.is_empty() {
            let rand_row = rand_item_from_vec(&game.board.available_moves.keys().collect());
            let rand_col = rand_item_from_vec(game.board.available_moves.get(rand_row).unwrap());

            game.board.move_cursor(rand_col as u16, *rand_row as u16);

            sleep(2000);
            game.play_move();
          } else {
            sleep(1000);
          }

          game.toggle_side();
          cur_turn = if game.cur_turn_side == WHITE { self.white } else { self.black };
        }
      }
      game.render_cur_turn_side();
      game.check_is_over();
      game.render_board();
      terminal.refresh().unwrap();
    }

    game.render_game_over(&mut offline_win);

    terminal.refresh().unwrap();

    terminal.getch();

    offline_win.delete();
  }
}

pub fn play_offline<Ctx>(terminal: &mut TerminalHandler, _: &mut Ctx, no_of_players: u8) -> Return {
  let cur_side = choose_side_win!(
    terminal, "Play Offline", if no_of_players == 1 { "Choose your side:" } else { "Choose Player 1 side: " }
  );

  use crate::game::offline_game::ParticipantType::{Player, Bot};

  if no_of_players == 2 {
    Offline{ black: Player, white: Player }
  } else {
    if cur_side == 'w' {
      Offline { black: Bot, white: Player }
    } else {
      Offline { black: Player, white: Bot }
    }
  }.begin_game(terminal);

  Return::ToRoot
}