pub mod board;

use board::Board;

use self::board::{BLACK, WHITE};

pub struct Game {
	pub board: Board
	// blackSide: ,
	// whiteSide: ,
	// curTurnRune: ,
	// gameState: ,
	// gameName: ,
	// stopDestructChan: ,
	// isDestructChanOpen: ,
}

impl Default for Game {
	fn default() -> Self {
		Game { board: Board::default() }
	}
}

impl Game {
  pub fn init_board(&mut self) {
    self.board.board[3][3] = BLACK;
    self.board.board[3][4] = WHITE;
    self.board.board[4][3] = WHITE;
    self.board.board[4][4] = BLACK;
  }
}