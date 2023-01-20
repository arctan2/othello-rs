use std::io::Write;

use crossterm::{style::{Color, Attribute}, event::KeyCode};
use serde::Deserialize;
use crate::{
  sleep, 
  termin::{
    terminal_window::Terminal,
    window::{Window, draw_elements, Position::*},
    elements::Text
  },
  menu::Return,
  game::macros::{choose_side_win}
};

pub fn create_game<W: Write>(terminal: &mut Terminal<W>, host_name: &str) -> Return {
  let host_side = choose_side_win!(terminal, "Create Game Online", "Choose your side:");

  #[derive(Deserialize)]
  struct CreateGameArgs {
    hostName: String,
    hostSide: char
  }
  Return::All
}

pub fn join_online_game() {
  #[derive(Deserialize, Debug)]
  struct Response {
    lobbyName: Option<String>,
    isLobbyFull: Option<bool>,
    err: bool,
    msg: String
  }
}
