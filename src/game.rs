pub mod board;
pub mod offline;

use std::io::Write;

use board::Board;
use crossterm::{style::Color, event::KeyCode};

use crate::termin::{window::{WindowRef, Window, Position::Coord}, terminal_window::Terminal, elements::Rectangle};

use self::board::{BLACK, WHITE, UP, FIX, DOWN, LEFT, RIGHT, Side};

pub struct Game {
	pub board: Board,
	// blackSide: ,
	// whiteSide: ,
	cur_turn_side: Side,
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
		Self { board: Board::new(board_container, board), cur_turn_side: WHITE }
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

	pub fn render_available_moves(&mut self) {
		let mut b = Rectangle::default().bg(Color::Red).size(2, 1);

		for (row_idx, row) in &self.board.available_moves {
			for col_idx in row {
				let x = col_idx * 4 + 2;
				let y = row_idx * 2 + 1;
				b.set_xy(x as u16, y as u16);
				self.board.board_container.draw_element(&b);
			}
		}

		self.board.board_container.render();
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
			self.board.render();
			self.render_available_moves();
			terminal.refresh().unwrap();
		}
	}
}