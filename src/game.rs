pub mod board;
pub mod offline_game;
mod online_game;
pub mod online_lobby;
pub mod macros;
pub mod socket;

use std::io::Write;

use board::Board;
use crossterm::{style::Color, event::KeyCode};

use crate::termin::{window::{WindowRef, Window, Position}, terminal_window::Terminal, elements::{Rectangle, Text}};

use self::board::{BLACK, WHITE, UP, FIX, DOWN, LEFT, RIGHT, Side};

#[derive(Debug)]
pub struct Game {
	pub board: Board,
	pub cur_turn_side_win: WindowRef,
	cur_turn_side: Side,
	render_cursor: bool,
	render_available_moves: bool,
	is_over: bool,
}

impl Game {
	pub fn new(mut win: WindowRef) -> Self {
		let (width, height) = (32 - 2, 15);
		let mut board_container = win.new_child(
			Window::default().size(width + 4, height + 2).bg(Color::Green).xy(1, 4)
		);
		let board = board_container.new_child(
			Window::default().size(32 - 2, 15).xy(2, 1).bg(Color::Green)
		);
		let points_win = win.new_child(
			Window::default().size(30, 1).xy(0, board_container.top() + board_container.height() + 2)
		);
		let cur_turn_side_win = win.new_child(Window::default().size(20, 1).xy(1, 1));

		Self {
			is_over: false,
			board: Board::new(board_container, board, points_win),
			cur_turn_side_win,
			cur_turn_side: BLACK,
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
		self.board.render_points();
	}

	pub fn render_available_moves(&mut self) {
		let mut b = Rectangle::default().bg(Color::Blue).size(2, 1);

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

	pub fn render_cur_turn_side(&mut self) {
		self.cur_turn_side_win.clear();
		let text_box = Text::default().text(if self.cur_turn_side == WHITE {
			"White's turn"
		} else {
			"Black's turn"
		});
		self.cur_turn_side_win.draw_element(&text_box);
		self.cur_turn_side_win.render();
	}

	pub fn set_cur_turn_side(&mut self, side: Side) {
		self.cur_turn_side = side;
	}

	pub fn toggle_side(&mut self) {
		self.cur_turn_side = if self.cur_turn_side == WHITE { BLACK } else { WHITE };
	}

	pub fn play_move(&mut self) {
		self.board.play_move(self.cur_turn_side);
	}

	pub fn render_game_over(&mut self, win: &mut WindowRef) {
    self.board.calc_points();
    let mut border = win.new_child(Window::default().bg(Color::Green).size(24, 7).xy(6, 7));
    let mut game_over_win = border.new_child(Window::default().size(20, 5));
    let mut text_box = Text::default().text("Game Over").position(game_over_win.rect(), Position::CenterH);
    game_over_win.set_position(Position::CenterB);

    text_box.set_xy_rel(0, 1);
    game_over_win.draw_element(&text_box);
    
    text_box.set_text(if self.board.black_points > self.board.white_points {
      "Black won"
    } else if self.board.white_points > self.board.black_points {
      "White won"
    } else {
      "Draw"
    });
		text_box.width_fit();
    text_box.set_xy_rel(0, 2);
    game_over_win.draw_element(&text_box);

    game_over_win.render_to_parent();
    border.render();
		border.delete();
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

	pub fn check_is_over(&mut self) {
		if !self.board.has_possible_moves(WHITE) && !self.board.has_possible_moves(BLACK) {
			self.is_over = true;
		}
	}
}
