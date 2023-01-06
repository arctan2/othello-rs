use std::io::Write;

use rand::Rng;

use crate::{termin::{terminal_window::Terminal, window::{Window, Position}, elements::Text}, sleep};

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

    game.board.calc_points();
    let mut game_over_win = offline_win.new_child(Window::default().size(20, 5).position(8, 7));
    let mut text_box = Text::default().text("Game Over").position(game_over_win.rect(), Position::CenterH);

    text_box.set_xy_rel(0, 1);
    game_over_win.draw_element(&text_box);

    
    text_box.set_text(if game.board.black_points > game.board.white_points {
      "Black won"
    } else if game.board.white_points > game.board.black_points {
      "White won"
    } else {
      "Draw"
    });
    text_box.set_xy_rel(0, 2);
    game_over_win.draw_element(&text_box);

    game_over_win.render();

    terminal.refresh().unwrap();

    terminal.getch();

    offline_win.delete();
  }
}
