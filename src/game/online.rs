use std::{net::TcpStream, io::Write, sync::mpsc::{self, Sender, Receiver}, thread};

use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::{style::Color, event::{KeyCode, Event}};
use reqwest::{Url, StatusCode};
use serde::{Deserialize, Serialize};
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};

use crate::{sleep, termin::{
  window::{Window, Position, WindowRef},
  terminal_window::{Terminal, TerminalHandler}, elements::{Rectangle, Text, InputBox}
}, custom_elements::DialogBox};

use super::{socket::{WS, SocketMsg, emit_json}, board::{WHITE, Side}};

pub struct Online {
  player_name: String,
  lobby: Lobby,
  pub side: char,
  pub player_id: String,
  pub game_id: String,
}

impl Online {
  pub fn new(win: &mut WindowRef) -> Self {
    Online { 
      lobby: Lobby::new(win),
      side: '\0', 
      player_id: "".to_string(),
      game_id: "".to_string(),
      player_name: "".to_string()
    }
  }
}


struct Lobby {
  white_name: String,
  black_name: String,
  lobby_win: WindowRef
}

macro_rules! render_seq {
  ($win:expr,{x: $x:expr,y: $y:expr},$first:expr,$first_gap:expr,$($el:expr,$gap:expr),+) => {
    $first.set_xy($x, $y);
    let (mut prev_left, mut prev_bottom) = ($x, $first.height() + $y + $first_gap);
    $win.render_element(&$first);
    $(
      $el.set_xy(prev_left, prev_bottom);
      $win.draw_element(&$el);
      (prev_left, prev_bottom) = ($el.x(), $el.height() + $el.y() + $gap);
    )+
  };
  ($win:expr,{x: $x:expr,y: $y:expr, gap: $gap:expr},$first:expr,$($el:expr),+) => {
    $first.set_xy($x, $y);
    let (mut prev_left, mut prev_bottom) = ($first.x(), $first.height() + $first.y() + $gap);
    $win.render_element(&$first);
    $(
      $el.set_xy(prev_left, prev_bottom);
      $win.draw_element(&$el);
      (prev_left, prev_bottom) = ($el.x(), $el.height() + $el.y() + $gap);
    )+
  };
}

impl Lobby {
  fn new(win: &mut WindowRef) -> Self {
    Self{
      white_name: "".to_string(),
      black_name: "".to_string(),
      lobby_win: win.new_child(Window::default().size(50, 15))
    }
  }

  fn render(&mut self, terminal: &mut TerminalHandler) {
    self.lobby_win.clear();
    let mut wrect = Rectangle::default().bg(Color::White).size(6, 3);
    let mut brect = Rectangle::default().bg(Color::Black).size(6, 3);
    let text = Text::default().text("Lobby").position(self.lobby_win.rect(), Position::CenterH).xy_rel(0, 1);
    self.lobby_win.set_bg(Color::Green);

    self.lobby_win.draw_element(&text);

    render_seq!(self.lobby_win, {x: 1, y: 3, gap: 1}, brect, wrect);

    self.lobby_win.draw_text(if self.black_name != "" {
      self.black_name.as_str()
    } else {
      "waiting to connect...."
    }, Position::Coord(9, 4));
    self.lobby_win.draw_text(if self.white_name != "" {
      self.white_name.as_str()
    } else {
      "waiting to connect...."
    }, Position::Coord(9, 8));
    self.lobby_win.draw_text("copied game link to clipboard", Position::Coord(9, 11));

    terminal.handler.draw_window(&self.lobby_win).unwrap();
    terminal.flush().unwrap();
  }
}

#[derive(Serialize)]
struct PlayerInfo {
  playerName: String,
  side: u8,
  isReconnect: bool,
  playerId: String
}

impl Online {
  pub fn set_player(mut self, side: Side, player_name: &str) -> Self {
    self.side = side;
    self.player_name = player_name.to_string();
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

  fn copy_link_to_clipboard(&self) {
    let url_str = "http://localhost:5000/api/join-game/".to_string() + &self.game_id;
    ClipboardContext::new().unwrap().set_contents(String::from(url_str).to_owned()).unwrap();
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
            playerName: self.player_name.clone(), side: self.side as u8, isReconnect: false, playerId: "".to_string()
          };

          match emit_json!(socket, "join-player-info", player_info) {
            Ok(_) => {
              dbox.info("joining lobby...");
              terminal.root.draw_element(&dbox);
              terminal.refresh().unwrap();
            },
            Err(e) => {
              dbox.error(e.as_str());
              terminal.root.draw_element(&dbox);
              terminal.refresh().unwrap();
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
          side: Option<u8> 
        }
        let msg: JoinPlayerInfoRes = msg.parse();
        if msg.err {
          dbox.error(&msg.msg);
          terminal.root.draw_element(&dbox);
        } else {

          self.lobby.render(terminal);
          self.copy_link_to_clipboard();
        }
      },
      "lobby-info" => {
        #[derive(Deserialize)]
        struct LobbyInfo {
          black: String,
          white: String
        }
        let info: LobbyInfo = msg.parse();
        self.lobby.black_name = info.black;
        self.lobby.white_name = info.white;
        self.lobby.render(terminal);
      },
      "countdown-begin" => {
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
    terminal.root.clear();
    let game_link = terminal.handle_input(|handler, root| -> String {
      let mut input_win = root.new_child(Window::default().size(50, 10));
      let label = Text::default().text("game link: ").xy(0, 2);
      let mut input = InputBox::default()
                      .max_len(50)
                      .position(label.x() + label.width(), label.y())
                      .size(25, 3).start_text((0, 0));

      input_win.set_xy_rel(2, 2);
      input_win.draw_element(&label);
      input_win.draw_text("Join Game", Position::CenterH);
      input_win.render();

      handler.draw_window(&root).unwrap();

      let new_name = input_win.read_string(&mut input, handler);
      input_win.delete();
      new_name
    });
    println!("{}", game_link);
    sleep(1000);
  }

  pub fn create_and_start(&mut self, terminal: &mut TerminalHandler) {
    let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
    dbox.info("creating game...");

    terminal.root.clear();
    terminal.root.draw_element(&dbox);
    terminal.refresh().unwrap();
    terminal.root.clear();

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
      Err(e) => { 
        dbox.error(e.as_str());
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
