use std::{net::TcpStream, io::Write};

use crossterm::style::Color;
use reqwest::{Url, StatusCode};
use serde::{Deserialize, Serialize};
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};

use crate::{sleep, termin::{
  window::{Window, Position},
  terminal_window::{Terminal, TerminalHandler}
}, custom_elements::DialogBox};

use super::socket::SocketMsg;

pub struct Online {
  pub player_name: String,
  pub side: char,
  pub player_id: String,
  pub game_id: String,
}

impl Default for Online {
  fn default() -> Self {
    Online { player_name: "".to_string(), side: '\0' , player_id: "".to_string(), game_id: "".to_string() }
  }
}

type WS = WebSocket<MaybeTlsStream<TcpStream>>;

struct Lobby {
  white_name: Option<String>,
  black_name: Option<String>
}

impl Lobby {
  fn new() -> Self {
    Self{ white_name: None, black_name: None }
  }

  fn join(&mut self) {
  }
}

impl Online {
  pub fn player_name(mut self, player_name: &str) -> Self {
    self.player_name = player_name.to_string(); 
    self
  }

  pub fn side(mut self, side: char) -> Self {
    self.side = side;
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

  fn join_lobby(&mut self, mut socket: WS, terminal: &mut TerminalHandler) {
    let mut game_verified = false;

    match socket.read_message().unwrap() {
      Message::Text(t) => {
        let msg = SocketMsg::from(t);
        match msg.event_name() {
          "game-verified" => {
            game_verified = msg.parse();
          },
          _ => ()
        }
      },
      _ => ()
    }
  }

  pub fn join_and_start(&mut self, terminal: &mut TerminalHandler) {
  }

  pub fn create_and_start(&mut self, terminal: &mut TerminalHandler) {
    let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
    dbox.info("creating game...");

    terminal.root.clear();
    dbox.render(&mut terminal.root);
    terminal.refresh().unwrap();

    match self.create_game() {
      Ok(()) => {
        dbox.info("connecting game...");
        dbox.render(&mut terminal.root);
        terminal.refresh().unwrap();

        match self.connect_socket() {
          Err(e) => {
            dbox.error(e.as_str());
            dbox.render(&mut terminal.root);
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
        dbox.render(&mut terminal.root);
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
