use crossterm::style::Color;

use crate::termin::{window::WindowRef, elements::{Rectangle, Text}};

pub const BLACK: char = 'b';
pub const WHITE: char = 'w';

pub const LEFT: i8 = -1;
pub const RIGHT: i8= 1;
pub const UP: i8  = -1;
pub const DOWN: i8  = 1;
pub const FIX: i8 = 0;

type Side = char;

struct Cursor {
	x: u16,
	y: u16,
	el: Rectangle
}

pub struct Board {
  pub board: [[Side; 8]; 8],
  pub board_container: WindowRef,
  pub board_win: WindowRef,
  cursor: Cursor
}

const TRAV_ARR: [[i8; 2]; 8] = [
	[UP, FIX],
	[DOWN, FIX],
	[FIX, LEFT],
	[FIX, RIGHT],
	[UP, RIGHT],
	[UP, LEFT],
	[DOWN, LEFT],
	[DOWN, RIGHT],
];

impl Board {
  pub fn new(board_container: WindowRef, board_win: WindowRef) -> Self {
    Self { 
      board: [['0'; 8]; 8],
      board_container, board_win,
      cursor: Cursor { x: 0, y: 0, el: Rectangle::default().bg(Color::Yellow).size(2, 1) }
    }
  }

  pub fn is_in_bounds(&self, row: i8, col: i8, row_len: i8, col_len: i8) -> bool {
    row >= 0 && row < row_len && col >= 0 && col < col_len
  }

  pub fn render(&mut self) {
    self.board_container.clear();
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut cell = Rectangle::default().size(2, 1).xy(pos_x, pos_y).bg(Color::Green);

    for row in self.board {
      pos_x = 0;
      for col in row {
        cell.set_xy(pos_x, pos_y);
        cell.set_bg(if col == WHITE {
          Color::White
        } else if col == BLACK {
          Color::Black
        } else {
          Color::Rgb { r: 80, g: 220, b: 120 }
        });

        self.board_win.draw_element(&cell);

        pos_x += 4;
      }
      pos_y += 2;
    }

    self.board_win.render_to_parent();
    self.render_cursor();
    self.board_container.render();
  }

  pub fn render_cursor(&mut self) {
    let x = self.cursor.x * 4 + 2;
    let y = self.cursor.y * 2 + 1;
    self.cursor.el.set_xy(x, y);
    self.board_container.draw_element(&self.cursor.el);
  }

  pub fn move_cursor(&mut self, x: u16, y: u16) {
    self.cursor.x = x;
    self.cursor.y = y;
  }

  pub fn move_cursor_rel(&mut self, x: i8, y: i8) {
    let mut new_x = x + self.cursor.x as i8;
    let mut new_y = y + self.cursor.y as i8;
    if new_x < 0 { new_x = 7; }
    if new_y < 0 { new_y = 7; }
    if new_x > 7 { new_x = 0; }
    if new_y > 7 { new_y = 0; }
    self.cursor.x = new_x as u16;
    self.cursor.y = new_y as u16;
  }

  pub fn cursor_xy(&self) -> (u16, u16) {
    (self.cursor.x, self.cursor.y)
  }

  pub fn traverse_from(&self, init_row: i8, init_col: i8, v_dir: i8, h_dir: i8, my_side: Side, opponent_side: Side) -> bool  {
    let mut row = init_row + v_dir;
    let mut col = init_col + h_dir;
    let (rl, cl) = (self.board.len() as i8, self.board[0].len() as i8);

    while self.is_in_bounds(row, col, rl, cl) && (self.board[row as usize][col as usize] == opponent_side) {
      row += v_dir;
      col += h_dir;
    }

    if !self.is_in_bounds(row, col, rl, cl) {
      row += v_dir * -1;
      col += h_dir * -1;
    }

    if self.board[row as usize][col as usize] == my_side && (col != init_col+h_dir || row != init_row+v_dir) {
      return true;
    }

    return false;
  }

  pub fn flip_from(&mut self, init_row: i8, init_col: i8, v_dir: i8, h_dir: i8, flip_from: Side, flip_to: Side) {
    let mut row = init_row + v_dir;
    let mut col = init_col + h_dir;
    let (rl, cl) = (self.board.len() as i8, self.board[0].len() as i8);

    while self.is_in_bounds(row, col, rl, cl) && (self.board[row as usize][col as usize] == flip_from) {
      self.board[row as usize][col as usize] = flip_to;
      row += v_dir;
      col += h_dir;
    }
  }

  pub fn traverse_and_flip(&mut self, i: i8, j: i8, my_side: Side, opponent_side: Side) -> bool {
    let mut is_flipped = false;

    for d in TRAV_ARR {
      let f = self.traverse_from(i, j, d[0], d[1], my_side, opponent_side);
      if f {
        self.flip_from(i, j, d[0], d[1], opponent_side, my_side);
      }
      is_flipped = f || is_flipped;
    }

    if is_flipped {
      self.board[i as usize][j as usize] = my_side;
    }

    return is_flipped;
  }

  pub fn has_possible_moves(&mut self, my_side: Side, opponent_side: Side) -> bool {
    for i in 0..self.board.len() {
      for j in 0..self.board[i].len() {
        if self.board[i][j] != my_side {
          continue;
        }
        for d in TRAV_ARR {
          if self.traverse_from(i as i8, j as i8, d[0], d[1], '0', opponent_side) {
            return true;
          }
        }
      }
    }
    return false;
  }

  pub fn get_points_for(&self, s: Side) -> i8 {
    let mut p = 0;

    for row in self.board {
      for cell in row {
        if cell == s {
          p += 1;
        }
      }
    }
    return p;
  }
}