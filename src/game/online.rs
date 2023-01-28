use std::{net::TcpStream, io::Write};

use crossterm::style::Color;
use reqwest::{Url, StatusCode};
use serde::{Deserialize, Serialize};
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};

use crate::{sleep, termin::{
  window::{Window, Position},
  terminal_window::{Terminal, TerminalHandler}
}, custom_elements::DialogBox};

use super::{socket::SocketMsg, board::{WHITE, Side}};

pub struct Online {
  player_name: String,
  lobby: Lobby,
  pub side: char,
  pub player_id: String,
  pub game_id: String,
}

impl Default for Online {
  fn default() -> Self {
    Online { 
      lobby: Lobby::new(),
      side: '\0', 
      player_id: "".to_string(),
      game_id: "".to_string(),
      player_name: "".to_string()
    }
  }
}

type WS = WebSocket<MaybeTlsStream<TcpStream>>;

macro_rules! emit {
  ($socket:expr,$e:expr) => {
    $socket.write_message(Message::Text($e))
  };
  ($socket:expr,$e:expr,$data:expr) => {
    match $socket.write_message(Message::Text($e.to_string())) {
      Ok(_) => {
        $socket.write_message($data)
      },
      Err(e) => Err(e)
    }
  };
}

macro_rules! emit_json {
  ($socket:expr,$e:expr,$data:expr) => {
    match $socket.write_message(Message::Text($e.to_string())) {
      Ok(_) => {
        match serde_json::to_string(&$data) {
          Ok(j) => match $socket.write_message(Message::Text(j)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
          },
          Err(e) => Err(e.to_string())
        }
      },
      Err(e) => Err(e.to_string())
    }
  };
}

struct Lobby {
  white_name: Option<String>,
  black_name: Option<String>
}

impl Lobby {
  fn new() -> Self {
    Self{ white_name: None, black_name: None }
  }
}

#[derive(Serialize)]
struct PlayerInfo {
  playerName: String,
  side: char,
  isReconnect: bool,
  playerId: String
}

impl Online {
  pub fn set_player(mut self, side: Side, player_name: &str) -> Self {
    self.side = side;
    self.player_name = player_name.to_string();
    if self.side == WHITE {
      self.lobby.white_name = Some(player_name.to_string());
    } else { 
      self.lobby.black_name = Some(player_name.to_string()); 
    }
    self
  }

  fn create_game(&mut self) -> Result<(), String> {
    #[derive(Serialize)]
    struct Host <'a> {
      hostName: &'a str,
      hostSide: i8
    }

    #[derive(Deserialize, Debug)]
    struct CreateGameResponse {
      msg: String,
      err: bool,
      gameId: Option<String>
    }
    
    let host = Host{hostName: &self.player_name, hostSide: self.side as i8};

    let req = reqwest::blocking::Client::new();

    let res = req.post("http://localhost:5000/api/create-lobby").json(&host).send();

    match res {
      Ok(res) => {
        let data: Result<CreateGameResponse, reqwest::Error> = res.json();
        match data {
          Ok(data) => {
            if data.err {
              return Err(data.msg);
            } else {
              self.game_id = data.gameId.unwrap_or("".to_string());
              return Ok(())
            }
          },
          Err(e) => return Err(e.to_string())
        }
      },
      Err(e) => return Err(e.to_string())
    }
  }

  fn connect_socket(&mut self) -> Result<WS, String> {
    let url_str = "ws://localhost:5000/api/join-game/".to_string() + &self.game_id;
    match tungstenite::connect(Url::parse(&url_str).unwrap()) {
      Ok((socket, res)) => {
        if res.status() != StatusCode::SWITCHING_PROTOCOLS {
          return Err("something went wrong".to_string());
        }
        return Ok(socket);
      },
      Err(e) => return Err(e.to_string())
    }
  }

  fn handle_join_lobby_socket(&mut self, socket: &mut WS, msg: SocketMsg, terminal: &mut TerminalHandler) -> bool {
    let mut dbox = DialogBox::new(35, 5)
                  .position(terminal.root.rect(), Position::Coord(5, 5))
                  .text("");

    match msg.event_name() {
      "game-verified" => {
        if !msg.parse::<bool>() {
          dbox.error("game not found :(");
          terminal.root.draw_element(&dbox);
        } else {
          let player_info = PlayerInfo{
            playerName: self.player_name.clone(), side: self.side, isReconnect: false, playerId: "".to_string()
          };
          match emit_json!(socket, "join-player-info", player_info) {
            Ok(_) => (),
            Err(e) => {
              dbox.error(e.as_str());
              terminal.root.draw_element(&dbox);
            }
          }
        }
      },
      "join-player-info-res" => {
        #[derive(Deserialize)]
        struct JoinPlayerInfoRes {
          err: bool,
          msg: String,
          playerId: Option<String>,
          side: Option<Side> 
        }
        let msg: JoinPlayerInfoRes = msg.parse();
        if msg.err {
          dbox.error(&msg.msg);
          terminal.root.draw_element(&dbox);
        } else {
          self.player_id = msg.playerId.unwrap();
          self.side = msg.side.unwrap();
        }
      },
      _ => ()
    }
    false
  }

  fn join_lobby(&mut self, mut socket: WS, terminal: &mut TerminalHandler) {
    terminal.clear();

    loop {
      match socket.read_message().unwrap() {
        Message::Text(t) => {
          let msg = SocketMsg::from(t);
          if self.handle_join_lobby_socket(&mut socket, msg, terminal) {
            break
          }
        },
        _ => ()
      }
    }
  }

  pub fn join_and_start(&mut self, terminal: &mut TerminalHandler) {
  }

  pub fn create_and_start(&mut self, terminal: &mut TerminalHandler) {
    let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
    dbox.info("creating game...");

    terminal.root.clear();
    terminal.root.draw_element(&dbox);
    terminal.refresh().unwrap();

    match self.create_game() {
      Ok(()) => {
        dbox.info("connecting game...");
        terminal.root.draw_element(&dbox);
        terminal.refresh().unwrap();

        match self.connect_socket() {
          Err(e) => {
            dbox.error(e.as_str());
            terminal.root.draw_element(&dbox);
            terminal.refresh().unwrap();
            terminal.getch();
          },
          Ok(socket) => {
            self.join_lobby(socket, terminal);
          }
        }
      },
      Err(_) => { 
        dbox.error("couldn't connect to server :(");
        terminal.root.draw_element(&dbox);
        terminal.refresh().unwrap();
        terminal.getch();
      }
    }
  }
}

pub fn is_game_exist(link: String) -> bool {
  #[derive(Deserialize, Debug)]
  struct Response {
    lobbyName: Option<String>,
    isLobbyFull: Option<bool>,
    err: bool,
    msg: String
  }
  false
}
