use crossterm::style::Color;

use crate::termin::{window::WindowRef, elements::Rectangle};

pub const BLACK: char = 'b';
pub const WHITE: char = 'w';

const left: i8 = -1;
const right: i8= 1;
const up: i8  = -1;
const down: i8  = 1;
const fix: i8 = 0;

type Side = char;

pub struct Board {
  pub board: [[Side; 8]; 8]
}

const TRAV_ARR: [[i8; 2]; 8] = [
	[up, fix],
	[down, fix],
	[fix, left],
	[fix, right],
	[up, right],
	[up, left],
	[down, left],
	[down, right],
];

impl Default for Board {
  fn default() -> Self {
    Self { board: [['0'; 8]; 8] }
  }
}

impl Board {
  pub fn isInBounds(&self, row: i8, col: i8, row_len: i8, col_len: i8) -> bool {
    row >= 0 && row < row_len && col >= 0 && col < col_len
  }

  pub fn print_board(&self, mut win: WindowRef) {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut cell = Rectangle::default().size(2, 1).position(pos_x, pos_y).bg(Color::Green);

    for row in self.board {
      pos_x = 0;
      for col in row {
        cell.set_pos(pos_x, pos_y);
        cell.set_bg(if col == WHITE {
          Color::White
        } else if col == BLACK {
          Color::Black
        } else {
          Color::Rgb { r: 80, g: 220, b: 120 }
        });

        win.draw_element(&cell);

        pos_x += 4;
      }
      pos_y += 2;
    }
  }

  pub fn traverseFrom(&self, initRow: i8, initCol: i8, vDir: i8, hDir: i8, mySide: Side, opponentSide: Side) -> bool  {
    let mut row = initRow + vDir;
    let mut col = initCol + hDir;
    let (rl, cl) = (self.board.len() as i8, self.board[0].len() as i8);

    while self.isInBounds(row, col, rl, cl) && (self.board[row as usize][col as usize] == opponentSide) {
      row += vDir;
      col += hDir;
    }

    if !self.isInBounds(row, col, rl, cl) {
      row += vDir * -1;
      col += hDir * -1;
    }

    if self.board[row as usize][col as usize] == mySide && (col != initCol+hDir || row != initRow+vDir) {
      return true;
    }

    return false;
  }

  pub fn flipFrom(&mut self, initRow: i8, initCol: i8, vDir: i8, hDir: i8, flipFrom: Side, flipTo: Side) {
    let mut row = initRow + vDir;
    let mut col = initCol + hDir;
    let (rl, cl) = (self.board.len() as i8, self.board[0].len() as i8);

    while self.isInBounds(row, col, rl, cl) && (self.board[row as usize][col as usize] == flipFrom) {
      self.board[row as usize][col as usize] = flipTo;
      row += vDir;
      col += hDir;
    }
  }

  pub fn traverseAndFlip(&mut self, i: i8, j: i8, mySide: Side, opponentSide: Side) -> bool {
    let mut isFlipped = false;

    for d in TRAV_ARR {
      let f = self.traverseFrom(i, j, d[0], d[1], mySide, opponentSide);
      if f {
        self.flipFrom(i, j, d[0], d[1], opponentSide, mySide);
      }
      isFlipped = f || isFlipped;
    }

    if isFlipped {
      self.board[i as usize][j as usize] = mySide;
    }

    return isFlipped;
  }

  pub fn hasPossibleMoves(&mut self, mySide: Side, opponentSide: Side) -> bool {
    for i in 0..self.board.len() {
      for j in 0..self.board[i].len() {
        if self.board[i][j] != mySide {
          continue;
        }
        for d in TRAV_ARR {
          if self.traverseFrom(i as i8, j as i8, d[0], d[1], '0', opponentSide) {
            return true;
          }
        }
      }
    }
    return false;
  }

  pub fn getPointsFor(&self, s: Side) -> i8 {
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