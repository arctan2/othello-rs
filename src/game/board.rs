use std::collections::HashMap;

use crossterm::style::Color;

use crate::termin::{window::WindowRef, elements::{Rectangle, Text}};

pub const BLACK: Side = 'b';
pub const WHITE: Side = 'w';
pub const EMPTY: Side = '0';

pub const LEFT: i8 = -1;
pub const RIGHT: i8= 1;
pub const UP: i8  = -1;
pub const DOWN: i8  = 1;
pub const FIX: i8 = 0;

pub type Side = char;

struct Cursor {
	x: u16,
	y: u16,
	el: Rectangle
}

pub struct Board {
  pub board: [[Side; 8]; 8],
  pub board_container: WindowRef,
  pub board_win: WindowRef,
  pub black_points: u8,
  pub white_points: u8,
  cursor: Cursor,
  pub available_moves: HashMap<usize, Vec<usize>>
}

const TRAV_ARR: [(i8, i8); 8] = [
	(UP, FIX),
	(DOWN, FIX),
	(FIX, LEFT),
	(FIX, RIGHT),
	(UP, RIGHT),
	(UP, LEFT),
	(DOWN, LEFT),
	(DOWN, RIGHT),
];

impl Board {
  pub fn new(board_container: WindowRef, board_win: WindowRef) -> Self {
    Self {
      board: [[EMPTY; 8]; 8],
      board_container, board_win,
      cursor: Cursor { x: 0, y: 0, el: Rectangle::default().bg(Color::Yellow).size(2, 1) },
      white_points: 0, black_points: 0,
      available_moves: HashMap::new()
    }
  }

  pub fn is_in_bounds(&self, row: i8, col: i8, row_len: i8, col_len: i8) -> bool {
    row >= 0 && row < row_len && col >= 0 && col < col_len
  }

  pub fn render(&mut self) {
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
  }

  pub fn place_cursor_on_legal_position(&mut self) {
    let row_idx = self.available_moves.keys().next().unwrap_or(&0);
    let col_idx = self.available_moves.get(row_idx).unwrap_or(&vec![0])[0];
    
    self.move_cursor(col_idx as u16, *row_idx as u16);
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

  pub fn move_cursor_rel(&mut self, dx: i8, dy: i8) {
    let mut row_idx = self.cursor.y as i8;
    let row = self.available_moves.get(&(row_idx as usize)).unwrap();
    let mut col_idx = row.iter().position(|x| *x == (self.cursor.x as usize)).unwrap() as i8;

    if dy == FIX {
      col_idx += dx;
      if col_idx < 0 { 
        loop {
          row_idx -= 1;
          if row_idx < 0 {
            row_idx = 7;
          }
          if self.available_moves.contains_key(&(row_idx as usize)) {
            col_idx = self.available_moves.get(&(row_idx as usize)).unwrap().len() as i8 - 1;
            break;
          }
        }
      } else if col_idx >= row.len() as i8 {
        loop {
          row_idx += 1;
          if row_idx > 7 {
            row_idx = 0;
          }
          if self.available_moves.contains_key(&(row_idx as usize)) {
            col_idx = 0;
            break;
          }
        }
      }
    }

    self.cursor.x = self.available_moves.get(&(row_idx as usize)).unwrap()[col_idx as usize] as u16;
    self.cursor.y = row_idx as u16;
  }

  pub fn cursor_xy(&self) -> (u16, u16) {
    (self.cursor.x, self.cursor.y)
  }

  pub fn traverse_from(&mut self, 
    init_row: i8, init_col: i8,
    v_dir: i8, h_dir: i8,
    my_side: Side, opponent_side: Side
  ) -> (bool, usize, usize) {
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

    (self.board[row as usize][col as usize] == my_side && (col != init_col+h_dir || row != init_row+v_dir),
      row as usize, 
      col as usize)
  }

  fn flip_from(&mut self, init_row: i8, init_col: i8, v_dir: i8, h_dir: i8, flip_from: Side, flip_to: Side) -> usize {
    let mut row = init_row + v_dir;
    let mut col = init_col + h_dir;
    let (rl, cl) = (self.board.len() as i8, self.board[0].len() as i8);
    let mut flipped_count: usize = 0;

    while self.is_in_bounds(row, col, rl, cl) && (self.board[row as usize][col as usize] == flip_from) {
      self.board[row as usize][col as usize] = flip_to;
      row += v_dir;
      col += h_dir;
      flipped_count += 1;
    }
    flipped_count
  }

  fn opponent_of(&self, p: Side) -> Side {
    if p == WHITE { BLACK } else { WHITE }
  }

  pub fn traverse_and_flip(&mut self, i: i8, j: i8, my_side: Side, opponent_side: Side) -> (bool, usize) {
    let mut is_flipped = false;
    let mut flipped_count = 0;

    for d in TRAV_ARR {
      let (f, _, _) = self.traverse_from(i, j, d.0, d.1, my_side, opponent_side);
      if f {
        flipped_count = self.flip_from(i, j, d.0, d.1, opponent_side, my_side);
      }
      is_flipped = f || is_flipped;
    }

    if is_flipped {
      self.board[i as usize][j as usize] = my_side;
    }

    return (is_flipped, flipped_count);
  }

  pub fn set_points(&mut self, b: u8, w: u8) {
    self.black_points = b;
    self.white_points = w;
  }

  pub fn play_move(&mut self, cur_turn: Side) {
    let opponent_side = self.opponent_of(cur_turn);
    let (cx, cy) = self.cursor_xy();
    self.board[cy as usize][cx as usize] = cur_turn;
    let (has_flipped, flipped_count) = self.traverse_and_flip(cy as i8, cx as i8, cur_turn, opponent_side);
    if !has_flipped { self.board[cy as usize][cx as usize] = EMPTY; }
    if flipped_count != 0 {
      if cur_turn == BLACK {
        self.set_points(self.black_points + flipped_count as u8 + 1, self.white_points - flipped_count as u8);
      } else {
        self.set_points(self.black_points - flipped_count as u8, self.white_points + flipped_count as u8 + 1);
      }
    }
  }

  pub fn has_possible_moves(&mut self, my_side: Side, opponent_side: Side) -> bool {
    for i in 0..self.board.len() {
      for j in 0..self.board[i].len() {
        if self.board[i][j] != my_side {
          continue;
        }

        for (y, x) in TRAV_ARR {
          let (f, _, _) = self.traverse_from(i as i8, j as i8, y, x, '0', opponent_side);
          if f {
            return true;
          }
        }
      }
    }
    return false;
  }

  pub fn calc_available_moves(&mut self, for_side: Side) {
    let opponent_side = self.opponent_of(for_side);
    let mut available_moves: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..self.board.len() {
      for j in 0..self.board[i].len() {
        if self.board[i][j] != for_side { continue; }
        for (vertic, horiz) in TRAV_ARR {
          let (f, r, c) = self.traverse_from(i as i8, j as i8, vertic, horiz, EMPTY, opponent_side);

          if f {
            match available_moves.get_mut(&r) {
              Some(row) => {
                if !row.contains(&c) {
                  row.push(c);
                }
              },
              None => { available_moves.insert(r, vec![c]); }
            }
          }
        }
      }
    }

    self.available_moves = available_moves;
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