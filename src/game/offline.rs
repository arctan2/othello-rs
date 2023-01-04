use std::io::Write;

use rand::Rng;

use crate::{termin::{terminal_window::Terminal, window::Window}, sleep};

use super::{Game, board::{WHITE, BLACK}};

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
  pub fn begin_game<W: Write>(&self, terminal: &mut Terminal<W>) {
    terminal.clear();
    let mut offline_win = terminal.root.new_child(Window::default().size(terminal.root.width(), terminal.root.height()));
    let mut game = Game::new(offline_win.clone());
    let mut cur_turn = self.white;

    game.init_board();

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

          game.cur_turn_side = if game.cur_turn_side == WHITE { BLACK } else { WHITE };
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

          game.cur_turn_side = if game.cur_turn_side == WHITE { BLACK } else { WHITE };
          cur_turn = if game.cur_turn_side == WHITE { self.white } else { self.black };
        }
      }
      game.check_is_over();
      game.render_board();
      terminal.refresh().unwrap();
    }

    offline_win.delete();
  }
}
