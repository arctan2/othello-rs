#![allow(non_snake_case)]

use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

use crate::{termin::{terminal_window::TerminalHandler, window::{Window, Position, WindowRef}}, custom_elements::DialogBox};
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

                let mut board: [[char;8];8] = [[0 as char;8];8];

                for i in 0..8 {
                  for j in 0..8 {
                    let cell = data.board[i][j];
                    if cell != 0 {
                      board[i][j] = data.board[i][j] as char;
                    }
                  }
                }

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

  pub async fn begin_game(&mut self, terminal: &mut TerminalHandler, mut socket: WS) {
    let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
    terminal.clear();

    dbox.info("Starting game...");
    terminal.root.draw_element(&dbox);
    terminal.refresh().unwrap();

    self.get_game_state(terminal, &mut dbox, &mut socket).await;

    while !self.game.is_over {
    }

    self.game.render_game_over(&mut self.online_win);

    terminal.refresh().unwrap();

    terminal.getch();

    self.online_win.delete();
  }
}
