#![allow(non_snake_case)]

use std::io::Error;

use crossterm::{
    event::{Event, EventStream, KeyCode},
    style::Color,
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_tungstenite::tungstenite::Message;

use super::{
    board::{Side, WHITE},
    chat::ChatSection,
    socket::{emit, SocketMsg, WS},
    Game,
};
use crate::{
    custom_elements::DialogBox,
    game::{board::EMPTY, chat::ChatMsg, socket::emit_json},
    sleep,
    termin::{
        self,
        terminal_window::TerminalHandler,
        window::{Position, Window, WindowRef},
    },
};

#[derive(Debug)]
pub struct OnlineGame {
    chat: ChatSection,
    game: Game,
    my_side: Side,
    is_cur_turn: bool,
    is_opponent_online: bool,
    online_win: WindowRef,
}

#[derive(Serialize, Deserialize)]
struct MoveDetails {
    rowIdx: u16,
    colIdx: u16,
}

enum GameStatus {
    GameOver(String),
    ChatMsg,
    WaitForReconnect,
    OpponentReconnect,
    Continue,
    RefreshTerminal
}

enum Control {
    Break,
    Continue,
    WaitForReconnect(bool)
}

#[derive(Copy, Clone)]
enum WindowMode {
    ChatMode,
    GameMode,
}

impl OnlineGame {
    pub fn new(opponent_name: String, my_side: Side, terminal: &mut TerminalHandler) -> Self {
        let mut online_win = terminal
            .root
            .new_child(Window::default().size(terminal.root.width(), terminal.root.height()));
        let game = Game::new(&mut online_win);
        Self {
            chat: ChatSection::new(&mut online_win, opponent_name),
            my_side,
            is_opponent_online: false,
            is_cur_turn: false,
            online_win,
            game,
        }
    }

    async fn get_game_state(
        &mut self,
        terminal: &mut TerminalHandler,
        dbox: &mut DialogBox,
        socket: &mut WS,
    ) {
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
                Ok(msg) => match msg {
                    Message::Text(t) => {
                        let msg = SocketMsg::from(t);
                        if msg.event_name() == "game-state-res" {
                            #[derive(Deserialize)]
                            struct GameStateRes {
                                board: [[u8; 8]; 8],
                                curTurn: u8,
                                blackPoints: u8,
                                whitePoints: u8,
                                isOpponentOnline: bool,
                            }
                            let data: GameStateRes = msg.parse();

                            let board = data.board.map(|row| {
                                row.map(|cell| if cell == 0 { EMPTY } else { cell as Side })
                            });

                            self.game.board.board = board;
                            self.game.board.black_points = data.blackPoints;
                            self.game.board.white_points = data.whitePoints;
                            self.is_cur_turn = data.curTurn as char == self.my_side;
                            self.is_opponent_online = data.isOpponentOnline;
                            return;
                        }
                    }
                    _ => (),
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
                    _ => None,
                },
                Err(_) => None,
            },
            None => None,
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

    fn play_move_local(&mut self) {
        self.game.play_move();
        self.game.toggle_side();
        self.game.render_cur_turn_side();
    }

    fn handle_socket_msg(&mut self, msg: SocketMsg) -> GameStatus {
        match msg.event_name() {
            "cur-turn" => {
                self.set_cur_turn_true();
                self.game.render_board();
                GameStatus::RefreshTerminal
            }
            "opponent-move" => {
                let opponent_move: MoveDetails = msg.parse();
                self.game
                    .board
                    .move_cursor(opponent_move.colIdx, opponent_move.rowIdx);
                self.play_move_local();
                GameStatus::Continue
            }
            "game-over" => GameStatus::GameOver(msg.data),
            "chat-msg" => {
                self.chat.receive(msg.parse());
                GameStatus::ChatMsg
            },
            "wait-for-opponent-reconnect" => GameStatus::WaitForReconnect,
            "opponent-reconnect" => GameStatus::OpponentReconnect,
            _ => GameStatus::Continue,
        }
    }

    pub fn handle_game_over(&mut self, msg: String) {
        if msg != "" {
            self.game.render_game_over(&mut self.online_win, &msg);
        } else {
            self.game.render_game_over(
                &mut self.online_win,
                if self.game.is_game_draw() {
                    "Draw"
                } else if self.my_side == WHITE {
                    if self.game.is_white_won() {
                        "you won :)"
                    } else {
                        "you lost :("
                    }
                } else {
                    if self.game.is_white_won() {
                        "you lost :("
                    } else {
                        "you won :)"
                    }
                },
            );
        }
    }

    async fn play_move(
        &mut self,
        terminal: &mut TerminalHandler,
        socket: &mut WS,
        dbox: &mut DialogBox,
    ) {
        let (col_idx, row_idx) = self.game.board.cursor_xy();
        let m = MoveDetails {
            colIdx: col_idx,
            rowIdx: row_idx,
        };

        self.play_move_local();
        self.set_cur_turn_false();

        self.game.render_board();
        terminal.refresh().unwrap();

        match emit_json!(socket, "move", m) {
            Err(_) => {
                dbox.error("connection lost :(");
                terminal.root.draw_element(dbox);
                terminal.refresh().unwrap();
                terminal.getch();
                return;
            }
            _ => (),
        }
    }

    async fn handle_kb_ev(
        &mut self,
        e: Option<Result<Event, Error>>,
        terminal: &mut TerminalHandler,
        socket: &mut WS,
        cur_window_mode: &mut WindowMode
    ) -> Control {
        use WindowMode::*;

        let mut dbox = DialogBox::new(35, 5).position(terminal.root.rect(), Position::Coord(5, 5));
        match cur_window_mode {
            GameMode => {
                if let Some(k) = self.to_keycode(e) {
                    match k {
                        KeyCode::Char('c') => {
                            terminal.clear();
                            self.chat.render();
                            terminal.refresh().unwrap();
                            self.chat.enable_cursor();
                            terminal.flush().unwrap();
                            *cur_window_mode = ChatMode;
                        },
                        KeyCode::Enter => {
                            if self.is_cur_turn {
                                self.play_move(terminal, socket, &mut dbox).await;
                            }
                        },
                        KeyCode::Esc => {
                            socket.close(None).await.unwrap();
                            return Control::Break
                        },
                        _ => {
                            self.game.keyboard_event(k);
                            self.game.render_board();
                            terminal.refresh().unwrap();
                        }
                    }
                }
            },

            ChatMode => {
                if let Some(e) = e {
                    if let Ok(e) = e {
                        match e {
                            Event::Key(k) => match k.code {
                                KeyCode::Enter => {
                                    let msg = self.chat.input_win.get_text_clone();
                                    if !msg.trim().is_empty() {
                                        self.chat.send(&msg);
                                        match emit!(socket, "chat-msg", &msg) {
                                            Ok(()) => (),
                                            Err(err) => {
                                                dbox.error(err.as_str());
                                                terminal.root.draw_element(&dbox);
                                                terminal.refresh().unwrap();
                                                terminal.getch();
                                            }
                                        }
                                    }
                                    self.chat.clear_input_win();
                                    terminal.draw_window(&self.chat.chat_msgs).unwrap();
                                    terminal.draw_window(self.chat.input_win.input_win()).unwrap();
                                },
                                KeyCode::Down => {
                                    self.chat.scroll_down();
                                    terminal.draw_window(&self.chat.chat_msgs).unwrap();
                                },
                                KeyCode::Up => {
                                    self.chat.scroll_up();
                                    terminal.draw_window(&self.chat.chat_msgs).unwrap();
                                },
                                KeyCode::Esc => {
                                    self.chat.disable_cursor();
                                    terminal.clear();
                                    self.game.render_board();
                                    self.game.render_cur_turn_side();
                                    terminal.refresh().unwrap();
                                    *cur_window_mode = GameMode;
                                },
                                _ => {
                                    self.chat.handle_kbd(e);
                                    self.chat.input_win.render();
                                    terminal.draw_window(self.chat.input_win.input_win()).unwrap();
                                }
                            },
                            _ => ()
                        }
                        self.chat.input_win.update_cursor();
                        terminal.flush().unwrap();
                    }
                }
            }
        }
        return Control::Continue
    }

    fn handle_socket_ev(
        &mut self,
        socket_ev: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
        terminal: &mut TerminalHandler,
        cur_window_mode: WindowMode
    ) -> Control {
        match socket_ev {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => match msg {
                    Message::Text(msg) => {
                        match self.handle_socket_msg(SocketMsg::from(msg)) {
                            GameStatus::GameOver(msg) => {
                                self.handle_game_over(msg);
                                terminal.refresh().unwrap();
                                terminal.getch();
                                return Control::Break
                            },
                            GameStatus::ChatMsg => {
                                if let WindowMode::ChatMode = cur_window_mode {
                                    terminal.draw_window(&self.chat.chat_msgs).unwrap();
                                    terminal.flush().unwrap();
                                    self.chat.input_win.update_rel_xy();
                                    self.chat.input_win.update_cursor();
                                }
                            },
                            GameStatus::WaitForReconnect => {
                                self.chat.set_recvr_is_online(false);
                                if let WindowMode::ChatMode = cur_window_mode {
                                    terminal.draw_window(&self.chat.chat_section).unwrap();
                                    terminal.flush().unwrap();
                                    self.chat.input_win.update_rel_xy();
                                    self.chat.input_win.update_cursor();
                                }
                                return Control::WaitForReconnect(true)
                            },
                            GameStatus::OpponentReconnect => {
                                self.chat.set_recvr_is_online(true);
                                if let WindowMode::ChatMode = cur_window_mode {
                                    terminal.draw_window(&self.chat.chat_section).unwrap();
                                    terminal.flush().unwrap();
                                    self.chat.input_win.update_rel_xy();
                                    self.chat.input_win.update_cursor();
                                }
                                return Control::WaitForReconnect(false)
                            }
                            GameStatus::RefreshTerminal => {
                                terminal.refresh().unwrap();
                            },
                            GameStatus::Continue => {
                                return Control::Continue
                            }
                        }
                    }
                    _ => ()
                },
                Err(_) => ()
            },
            None => ()
        }
        return Control::Continue
    }

    pub async fn begin_game(&mut self, terminal: &mut TerminalHandler, mut socket: WS) {
        self.chat.set_recvr_is_online(true);
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

        use WindowMode::*;

        let mut cur_window_mode = GameMode;
        let mut event = EventStream::new();
        let mut wait_for_reconn = false;
        let mut wait_for_reconn_win = self.online_win.new_child(
            Window::default()
            .size(40, 1)
            .xy(0, 0)
        );
        let mut cur_wait_time = 20;

        use tokio::time::{self, Instant, Duration};

        let timer = time::sleep(Duration::from_millis(1000));
        tokio::pin!(timer);

        while !self.game.is_over {
            if wait_for_reconn {
                timer.as_mut().reset(Instant::now() + Duration::from_millis(1000));
                select! {
                    e = event.next() => {
                        match self.handle_kb_ev(e, terminal, &mut socket, &mut cur_window_mode).await {
                            Control::Break => break,
                            _ => ()
                        }
                    },
                    socket_ev = socket.next() => {
                        match self.handle_socket_ev(socket_ev, terminal, cur_window_mode) {
                            Control::Break => break,
                            Control::WaitForReconnect(w) => {
                                wait_for_reconn = w;
                                cur_wait_time = 20;
                                wait_for_reconn_win.clear();
                                terminal.draw_window(&wait_for_reconn_win).unwrap();
                                terminal.flush().unwrap();
                            }
                            _ => ()
                        }
                    },
                    () = &mut timer => {
                        if cur_wait_time == 9 {
                            wait_for_reconn_win.clear();
                        }
                        if cur_wait_time != 0 {
                            let t = &("waiting for opponent reconnection ".to_string() + &cur_wait_time.to_string());
                            wait_for_reconn_win.draw_text(t, Position::Coord(0, 0));
                            terminal.draw_window(&wait_for_reconn_win).unwrap();
                            terminal.flush().unwrap();
                        }

                        if cur_wait_time == -10 {
                            self.handle_game_over("connection was closed.".to_string());
                            break
                        }
                        cur_wait_time -= 1;
                    }
                };
            } else {
                select! {
                    e = event.next() => {
                        match self.handle_kb_ev(e, terminal, &mut socket, &mut cur_window_mode).await {
                            Control::Break => break,
                            _ => ()
                        }
                    },
                    socket_ev = socket.next() => {
                        match self.handle_socket_ev(socket_ev, terminal, cur_window_mode) {
                            Control::Break => break,
                            Control::WaitForReconnect(w) => {
                                wait_for_reconn = w;
                            }
                            _ => ()
                        }
                    }
                };
            }
        }

        self.online_win.delete();
    }
}
