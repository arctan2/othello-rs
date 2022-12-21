pub mod board;
pub mod offline;

use std::io::Write;

use board::Board;
use crossterm::{style::Color, event::KeyCode};

use crate::termin::{window::{WindowRef, Window, Position::Coord}, terminal_window::Terminal};

use self::board::{BLACK, WHITE, UP, FIX, DOWN, LEFT, RIGHT};

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
	pub fn new(mut win: WindowRef) -> Self {
		let (width, height) = (32 - 2, 15);
		let mut board_container = win.new_child(
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

	pub fn enable_cursor_movement<W: Write>(&mut self, terminal: &mut Terminal<W>) {
		loop {
			match terminal.getch() {
				KeyCode::Up => self.board.move_cursor_rel(FIX, UP),
				KeyCode::Down => self.board.move_cursor_rel(FIX, DOWN),
				KeyCode::Left => self.board.move_cursor_rel(LEFT, FIX),
				KeyCode::Right => self.board.move_cursor_rel(RIGHT, FIX),
				KeyCode::Enter => break,
				_ => ()
			}
			self.board.render_cursor();
			self.board.render();
			terminal.refresh().unwrap();
		}
	}
}