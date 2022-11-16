pub mod board;

use board::Board;
use crossterm::style::Color;

use crate::termin::{window::{WindowRef, Window, Position::Coord}};

use self::board::{BLACK, WHITE};

pub struct Game {
	pub board: Board,
	// blackSide: ,
	// whiteSide: ,
	// curTurnRune: ,
	// gameState: ,
	// gameName: ,
	// stopDestructChan: ,
	// isDestructChanOpen: ,
}

impl Game {
	pub fn new(mut root_win: WindowRef) -> Self {
		let (width, height) = (32 - 2, 15);
		let mut board_container = root_win.new_child(
			Window::default().size(width + 4, height + 2).bg(Color::Green)
		);
		let board = board_container.new_child(
			Window::default().size(32 - 2, 15).position(2, 1).bg(Color::Green)
		);
		Self { board: Board::new(board_container, board) }
	}

  pub fn init_board(&mut self) {
    self.board.board[3][3] = BLACK;
    self.board.board[3][4] = WHITE;
    self.board.board[4][3] = WHITE;
    self.board.board[4][4] = BLACK;
  }

	pub fn render_board(&mut self) {
		self.board.board_container.clear();
		self.board.render();
	}
}