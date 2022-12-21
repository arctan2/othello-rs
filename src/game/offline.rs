use std::io::Write;

use crate::termin::{terminal_window::Terminal, window::Window};

use super::Game;

pub enum ParticipantType {
  Bot,
  Player
}

pub struct Offline {
  pub black: ParticipantType,
  pub white: ParticipantType
}

impl Offline {
  pub fn begin_game<W: Write>(&self, terminal: &mut Terminal<W>) {
    terminal.clear();
    let mut offline_win = terminal.root.new_child(Window::default().size(terminal.root.width(), terminal.root.height()));

    let mut game = Game::new(offline_win.clone());
    game.init_board();
    game.render_board();
    terminal.refresh().unwrap();

    game.enable_cursor_movement(terminal);

    offline_win.delete();
  }
}
