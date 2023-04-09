#![allow(non_snake_case)]

use std::io::Error;

use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::{style::Color, event::{EventStream, KeyCode, Event}};
use futures_util::{stream::StreamExt, SinkExt};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    custom_elements::DialogBox,
    game::macros::render_seq,
    sleep,
    termin::{
        elements::{InputWindow, Rectangle, Text},
        terminal_window::TerminalHandler,
        window::{Position, Window, WindowRef},
    },
};

use super::{
    board::Side,
    online_game::OnlineGame,
    socket::{emit_json, SocketMsg, WS},
};

#[derive(Debug)]
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
            player_name: "".to_string(),
        }
    }
}

#[derive(Debug)]
struct Lobby {
    name: String,
    white_name: String,
    black_name: String,
    lobby_win: WindowRef,
    game_status_win: WindowRef,
}

const HLINK: &str = "https://othellojs.onrender.com";
const WLINK: &str = "wss://othellojs.onrender.com";

macro_rules! api_link {
    ($post:expr) => { &(HLINK.to_string() + $post) };
    ($pre:literal, $post:expr) => {
        if $pre == "w" {
            WLINK.to_string() + $post
        } else {
            HLINK.to_string() + $post
        }
    }
}

impl Lobby {
    fn new(win: &mut WindowRef) -> Self {
        let mut lobby_win = win.new_child(Window::default().size(50, 15));
        lobby_win.set_position(Position::CenterB);
        Self {
            name: "".to_string(),
            white_name: "".to_string(),
            black_name: "".to_string(),
            game_status_win: lobby_win
                .new_child(Window::default().bg(Color::Green).size(40, 1).xy(9, 11)),
            lobby_win,
        }
    }

    fn render(&mut self, terminal: &mut TerminalHandler) {
        terminal.clear();
        terminal.refresh().unwrap();
        self.lobby_win.clear();
        let mut wrect = Rectangle::default().bg(Color::White).size(6, 3);
        let mut brect = Rectangle::default().bg(Color::Black).size(6, 3);
        let text = Text::default()
            .text("Lobby")
            .position(self.lobby_win.rect(), Position::CenterH)
            .xy_rel(0, 1);
        self.lobby_win.set_bg(Color::Green);

        self.lobby_win.draw_element(&text);

        render_seq!(self.lobby_win, {x: 1, y: 3, gap: 1}, brect, wrect);

        self.lobby_win.draw_text(
            if self.black_name != "" {
                self.black_name.as_str()
            } else {
                "waiting to connect...."
            },
            Position::Coord(9, 4),
        );
        self.lobby_win.draw_text(
            if self.white_name != "" {
                self.white_name.as_str()
            } else {
                "waiting to connect...."
            },
            Position::Coord(9, 8),
        );
        self.game_status_win
            .draw_text("copied game link to clipboard", Position::Coord(0, 0));
        self.game_status_win.render_to_parent();

        terminal.draw_window(&self.lobby_win).unwrap();
        terminal.flush().unwrap();
    }
}

#[derive(Serialize)]
struct PlayerInfo {
    playerName: String,
    side: u8,
    isReconnect: bool,
    playerId: String,
}

enum NextGameState {
    StartGame,
    Error(String),
    Continue
}

impl Online {
    pub fn set_player(mut self, side: Side, player_name: &str) -> Self {
        self.side = side;
        self.player_name = player_name.to_string();
        self
    }

    fn create_game(&mut self) -> Result<(), String> {
        #[derive(Serialize)]
        struct Host<'a> {
            hostName: &'a str,
            hostSide: i8,
        }

        #[derive(Deserialize, Debug)]
        struct CreateGameResponse {
            msg: String,
            err: bool,
            gameId: Option<String>,
        }

        let host = Host {
            hostName: &self.player_name,
            hostSide: self.side as i8,
        };

        let req = reqwest::blocking::Client::new();

        let res = req
            .post(api_link!("/api/create-lobby"))
            .json(&host)
            .send();

        match res {
            Ok(res) => {
                let data: Result<CreateGameResponse, reqwest::Error> = res.json();
                match data {
                    Ok(data) => {
                        if data.err {
                            return Err(data.msg);
                        } else {
                            self.game_id = data.gameId.unwrap_or("".to_string());
                            return Ok(());
                        }
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    async fn connect_socket(&mut self) -> Result<WS, String> {
        let url_str = api_link!("w", &("/api/join-game/".to_string() + &self.game_id));
        match tokio_tungstenite::connect_async(Url::parse(&url_str).unwrap()).await {
            Ok((socket, res)) => {
                if res.status() != StatusCode::SWITCHING_PROTOCOLS {
                    return Err("something went wrong".to_string());
                }
                return Ok(socket);
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    fn copy_link_to_clipboard(&self) {
        let url_str = api_link!(&("/join-game/".to_string() + &self.game_id));
        ClipboardContext::new()
            .unwrap()
            .set_contents(String::from(url_str).to_owned())
            .unwrap();
    }

    async fn handle_lobby_socket(
        &mut self,
        socket: &mut WS,
        msg: SocketMsg,
        dbox: &mut DialogBox,
        terminal: &mut TerminalHandler,
    ) -> NextGameState {
        match msg.event_name() {
            "game-verified" => {
                if !msg.parse::<bool>() {
                    return NextGameState::Error("game not found :(".to_string());
                } else {
                    let player_info = PlayerInfo {
                        playerName: self.player_name.clone(),
                        side: self.side as u8,
                        isReconnect: false,
                        playerId: "".to_string(),
                    };

                    match emit_json!(socket, "join-player-info", player_info) {
                        Ok(_) => {
                            dbox.info("joining lobby...");
                            terminal.root.draw_element(dbox);
                            terminal.refresh().unwrap();
                        }
                        Err(e) => {
                            return NextGameState::Error(e.to_string());
                        }
                    }
                }
            }
            "join-player-info-res" => {
                #[derive(Deserialize)]
                struct JoinPlayerInfoRes {
                    err: bool,
                    msg: String,
                    playerId: Option<String>,
                    side: Option<u8>,
                }
                let msg: JoinPlayerInfoRes = msg.parse();

                if msg.err {
                    return NextGameState::Error(msg.msg);
                } else {
                    self.lobby.render(terminal);
                    self.copy_link_to_clipboard();
                    self.side = msg.side.unwrap() as Side;
                    self.player_id = msg.playerId.unwrap();
                }
            }
            "lobby-info" => {
                #[derive(Deserialize)]
                struct LobbyInfo {
                    black: String,
                    white: String,
                }
                let info: LobbyInfo = msg.parse();
                self.lobby.black_name = info.black;
                self.lobby.white_name = info.white;
                self.lobby.render(terminal);
            }
            "countdown-begin" => {
                self.lobby.game_status_win.clear();
                sleep(1000);
                for i in (0..=5).rev() {
                    let t = "Game will start in ".to_string() + &i.to_string();
                    self.lobby
                        .game_status_win
                        .draw_text(&t, Position::Coord(0, 0));
                    terminal.draw_window(&self.lobby.game_status_win).unwrap();
                    terminal.flush().unwrap();
                    sleep(1000);
                }
                return NextGameState::StartGame;
            }
            _ => (),
        }
        NextGameState::Continue
    }

    fn to_keycode(&self, e: Option<Result<Event, Error>>) -> Option<KeyCode> {
        match e {
            Some(e) => match e {
                Ok(e) => match e {
                    Event::Key(e) => Some(e.code),
                    _ => None,
                },
                Err(_) => None,
            },
            None => None,
        }
    }

    async fn join_lobby(&mut self, mut socket: WS, terminal: &mut TerminalHandler) {
        terminal.clear();
        let mut dbox = DialogBox::new(35, 5)
            .position(terminal.root.rect(), Position::Coord(5, 5))
            .text("");
        let mut event = EventStream::new();

        loop {
            tokio::select!{
                e = event.next() => {
                    if let Some(k) = self.to_keycode(e) {
                        match k {
                            KeyCode::Esc => {
                                socket.close(None).await.unwrap();
                                return
                            },
                            _ => ()
                        }
                    }
                },
                socket_ev = socket.next() => {
                    match socket_ev {
                        Some(maybe_msg) => match maybe_msg {
                            Ok(msg) => match msg {
                                Message::Text(msg) => {
                                    match self.handle_lobby_socket(&mut socket, SocketMsg::from(msg), &mut dbox, terminal).await {
                                        NextGameState::Continue => continue,
                                        NextGameState::StartGame => {
                                            std::mem::drop(event); 
                                            self.lobby.game_status_win.delete();
                                            self.lobby.lobby_win.delete();

                                            let mut game = OnlineGame::new(
                                                if self.side == 'w' {
                                                    self.lobby.black_name.clone()
                                                } else {
                                                    self.lobby.white_name.clone()
                                                },
                                                self.side,
                                                terminal,
                                            );

                                            game.begin_game(socket).await;
                                            return;
                                        }
                                        NextGameState::Error(e) => {
                                            dbox.error(&e);
                                            terminal.root.draw_element(&dbox);
                                            terminal.refresh().unwrap();
                                            terminal.getch();
                                            return;
                                        }
                                    }
                                }
                                _ => ()
                            },
                            Err(_) => ()
                        },
                        None => ()
                    }
                }
            }
        }
    }

    pub fn connect_game(&mut self, mut dbox: DialogBox, terminal: &mut TerminalHandler) {
        dbox.info("connecting game...");
        terminal.root.draw_element(&dbox);
        terminal.refresh().unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.connect_socket().await {
                Err(e) => {
                    dbox.error(e.as_str());
                    terminal.root.draw_element(&dbox);
                    terminal.refresh().unwrap();
                    terminal.getch();
                }
                Ok(socket) => {
                    self.join_lobby(socket, terminal).await;
                }
            }
        });
    }

    pub fn join_and_start(&mut self, terminal: &mut TerminalHandler) {
        terminal.root.clear();
        let game_link = terminal.handle_input(|handler, root| -> String {
            let mut input_win = root.new_child(Window::default().size(50, 10));
            let label = Text::default().text("game link: ").xy(0, 2);
            let mut input = InputWindow::from(
                &mut input_win,
                Window::default()
                    .xy(label.x() + label.width(), label.y())
                    .size(25, 3),
            )
            .start_text((0, 0))
            .max_len(65);

            input_win.set_xy_rel(2, 2);
            input_win.draw_element(&label);
            input_win.draw_text("Join Game", Position::CenterH);
            input_win.render();

            handler.draw_window(&root).unwrap();

            let new_name = input.read_string(handler);
            input_win.delete();
            new_name
        });

        let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
        dbox.info("checking game existence...");
        terminal.root.clear();
        terminal.root.draw_element(&dbox);
        terminal.refresh().unwrap();
        terminal.root.clear();

        self.extract_game_id(game_link);

        match self.is_game_exist() {
            Ok(is_exist) => {
                if is_exist {
                    self.connect_game(dbox, terminal);
                } else {
                    dbox.error("game doesn't exist!");
                    terminal.root.draw_element(&dbox);
                    terminal.refresh().unwrap();
                    terminal.getch();
                }
            }
            Err(e) => {
                dbox.error(e.as_str());
                terminal.root.draw_element(&dbox);
                terminal.refresh().unwrap();
                terminal.getch();
            }
        }
    }

    fn extract_game_id(&mut self, link: String) {
        match link
            .trim_end_matches("/")
            .split("/")
            .collect::<Vec<&str>>()
            .last()
        {
            Some(last) => {
                self.game_id = last.to_string();
            }
            None => (),
        }
    }

    fn is_game_exist(&mut self) -> Result<bool, String> {
        if self.game_id.is_empty() {
            return Err("invalid link.".to_string());
        }

        #[derive(Deserialize, Debug)]
        struct Response {
            lobbyName: Option<String>,
            isLobbyFull: Option<bool>,
            err: bool,
            msg: String,
        }

        let req = reqwest::blocking::Client::new();
        let link = api_link!(&("/api/game-info/".to_string() + &self.game_id));

        let res = req.get(link).send();

        match res {
            Ok(res) => {
                let data: Result<Response, reqwest::Error> = res.json();
                match data {
                    Ok(data) => {
                        if data.err {
                            return Err(data.msg);
                        } else {
                            if data.isLobbyFull.unwrap() {
                                return Err("lobby full!".to_string());
                            } else {
                                self.lobby.name = data.lobbyName.unwrap();
                                return Ok(true);
                            }
                        }
                    },
                    Err(e) => return Err(e.to_string()),
                }
            },
            Err(e) => return Err(e.to_string()),
        }
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
                self.connect_game(dbox, terminal);
            }
            Err(e) => {
                dbox.error(e.as_str());
                terminal.root.draw_element(&dbox);
                terminal.refresh().unwrap();
                terminal.getch();
            }
        }
    }
}
