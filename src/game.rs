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
	render_cursor: bool,
	render_available_moves: bool,
	is_over: bool,
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
		Self {
			is_over: false,
			board: Board::new(board_container, board),
			cur_turn_side: WHITE,
			render_cursor: false,
			render_available_moves: false
		}
	}

  pub fn init_board(&mut self) {
    self.board.board[3][3] = BLACK;
    self.board.board[3][4] = WHITE;
    self.board.board[4][3] = WHITE;
    self.board.board[4][4] = BLACK;
		self.board.move_cursor(3, 2);
		self.board.black_points = 2;
		self.board.white_points = 2;
  }

	pub fn render_board(&mut self) {
		self.board.board_container.clear();
		self.board.render();
		if self.render_available_moves {
			self.render_available_moves();
		}
		if self.render_cursor {
			self.board.render_cursor();
		}

		self.board.board_container.render();
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

	pub fn play_move(&mut self) {
		self.board.play_move(self.cur_turn_side);
	}

	pub fn enable_cursor_movement<W: Write>(&mut self, terminal: &mut Terminal<W>) {
		self.render_cursor = true;
		self.render_available_moves = true;
		self.render_board();
		terminal.refresh().unwrap();
		loop {
			match terminal.getch() {
				KeyCode::Up => self.board.move_cursor_rel(FIX, UP),
				KeyCode::Down => self.board.move_cursor_rel(FIX, DOWN),
				KeyCode::Left => self.board.move_cursor_rel(LEFT, FIX),
				KeyCode::Right => self.board.move_cursor_rel(RIGHT, FIX),
				KeyCode::Enter => {
					self.play_move();
					break;
				},
				KeyCode::Esc => break,
				_ => ()
			}
			self.render_board();
			terminal.refresh().unwrap();
		}
		self.render_cursor = false;
		self.render_available_moves = false;
	}
}
