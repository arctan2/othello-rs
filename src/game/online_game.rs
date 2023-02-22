#![allow(non_snake_case)]

use std::io::Error;

use crossterm::event::{EventStream, Event, KeyCode};
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt, FutureExt};

use crate::{termin::{terminal_window::TerminalHandler, window::{Window, Position, WindowRef}}, custom_elements::DialogBox, game::{board::EMPTY, socket::emit_json}, sleep};
use super::{Game, socket::{WS, emit, SocketMsg}, board::Side};

#[derive(Debug)]
pub struct OnlineGame {
  opponent_name: String,
  my_side: Side,
  is_cur_turn: bool,
  is_opponent_online: bool,
  game: Game,
  online_win: WindowRef
}

#[derive(Serialize, Deserialize)]
struct MoveDetails {
  rowIdx: u16,
  colIdx: u16
}

impl OnlineGame {
  pub fn new(opponent_name: String, my_side: Side, terminal: &mut TerminalHandler) -> Self{
    let online_win = terminal.root.new_child(Window::default().size(terminal.root.width(), terminal.root.height()));
    let game = Game::new(online_win.clone());
    Self {opponent_name, my_side, is_opponent_online: false, is_cur_turn: false, online_win, game }
  }

  async fn get_game_state(&mut self, terminal: &mut TerminalHandler, dbox: &mut DialogBox, socket: &mut WS) {
    match emit!(socket, "game-state") {
      Ok(()) => (),
      Err(e) => {
        dbox.error(&e);
        terminal.root.draw_element(dbox);
        terminal.refresh().unwrap();
      }
    }

    loop {
      let msg = socket.next().await.unwrap();
      match msg {
        Ok(msg) => {
          match msg {
            Message::Text(t) => {
              let msg = SocketMsg::from(t);
              if msg.event_name() == "game-state-res" {
                #[derive(Deserialize)]
                struct GameStateRes {
                  board: [[u8;8];8],
			            curTurn: u8,
			            blackPoints: u8,
			            whitePoints: u8,
			            isOpponentOnline: bool,
                }
                let data: GameStateRes = msg.parse();

                let board = data.board.map(|row| row.map(|cell| if cell == 0 { EMPTY } else { cell as Side }));

                self.game.board.board = board;
                self.game.board.black_points = data.blackPoints;
                self.game.board.white_points = data.whitePoints;
                self.is_cur_turn = data.curTurn as char == self.my_side;
                self.is_opponent_online = data.isOpponentOnline;
                return;
              }
            },
            _ => ()
          }
        },
        Err(e) => {
          dbox.error(&e.to_string());
          terminal.root.draw_element(dbox);
          terminal.refresh().unwrap();
          terminal.getch();
          return;
        }
      }
    }
  }

  fn to_keycode(&self, e: Option<Result<Event, Error>>) -> Option<KeyCode> {
    match e {
      Some(e) => match e {
        Ok(e) => match e {
          Event::Key(e) => Some(e.code),
          _ => None
        },
        Err(_) => None
      },
      None => None
    }
  }

  fn set_cur_turn_true(&mut self) {
    self.is_cur_turn = true;
    self.game.render_cursor = true;
    self.game.render_available_moves = true;
    self.game.board.calc_available_moves(self.my_side);
    if !self.game.board.available_moves.is_empty() {
      self.game.board.place_cursor_on_legal_position();
    }
  }

  fn set_cur_turn_false(&mut self) {
    self.is_cur_turn = false;
    self.game.render_cursor = false;
    self.game.render_available_moves = false;
  }

  fn play_move(&mut self) {
    self.game.play_move();
    self.game.toggle_side();
  }

  fn handle_socket_msg(&mut self, terminal: &mut TerminalHandler, msg: SocketMsg) {
    match msg.event_name() {
	    "cur-turn" => {
        self.set_cur_turn_true();

        self.game.render_board();
        terminal.refresh().unwrap();
      },
      "opponent-move" => {
        let opponent_move: MoveDetails = msg.parse();
        self.game.board.move_cursor(opponent_move.colIdx, opponent_move.rowIdx);
        self.play_move();
      },
      "wait-for-opponent-reconnect" => {
      },
      "chat-msg" => {
      },
      _ => ()
    }
  }

  pub async fn begin_game(&mut self, terminal: &mut TerminalHandler, mut socket: WS) {
    let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
    terminal.clear();

    dbox.info("Starting game...");
    terminal.root.draw_element(&dbox);
    terminal.refresh_clear().unwrap();

    self.get_game_state(terminal, &mut dbox, &mut socket).await;

    if self.is_cur_turn {
      self.set_cur_turn_true();
    }

    self.game.render_board();
    self.game.render_cur_turn_side();
    terminal.refresh().unwrap();

    while !self.game.is_over {
      let mut event = EventStream::new();

      select! {
        e = event.next() => {
          if e.is_none() || !self.is_cur_turn {
            continue;
          }

          if let Some(k) = self.to_keycode(e) {
            match k {
              KeyCode::Enter => {
                if !self.is_cur_turn {
                  continue; 
                }
                let (col_idx, row_idx) = self.game.board.cursor_xy();
                let m = MoveDetails{colIdx: col_idx, rowIdx: row_idx};

                self.play_move();
                self.set_cur_turn_false();

                self.game.render_board();
                terminal.refresh().unwrap();

                match emit_json!(socket, "move", m) {
                  Err(_) => {
                    dbox.error("connection lost :(");
                    terminal.root.draw_element(&dbox);
                    terminal.refresh().unwrap();
                    terminal.getch();
                    return;
                  },
                  _=> ()
                } 
              },
              _ => {
                self.game.keyboard_event(k);
                self.game.render_board();
                terminal.refresh().unwrap();
              }
            }
          }
        },
        socket_ev = socket.next() => {
          match socket_ev {
            Some(maybe_msg) => match maybe_msg {
              Ok(msg) => match msg {
                  Message::Text(msg) => self.handle_socket_msg(terminal, SocketMsg::from(msg)),
                  _ => ()
              },
              Err(_) => ()
            }
            None => ()
          }
        }
      };
    }


    self.game.render_game_over(&mut self.online_win);

    terminal.refresh().unwrap();

    terminal.getch();

    self.online_win.delete();
  }
}
