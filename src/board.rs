pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
pub const N_IN_A_ROW: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Counter {
    PlayerCounter(Player),
    NoCounter,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Winner {
    WinningPlayer(Player),
    Draw,
    NoWinner,
}

#[derive(Debug)]
pub struct PlayerMove {
    pub player: Player,
    pub column: usize,
}

#[derive(Debug)]
pub enum MoveError {
    GameOver,
    NotYourTurn,
    InvalidColumn,
    ColumnFull,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub turn: Player,
    pub winner: Winner,
    pub top_spot: [usize; BOARD_WIDTH],
    pub matrix: [[Counter; BOARD_HEIGHT]; BOARD_WIDTH],
}

pub fn default_board() -> Board {
    Board {
        turn: Player::Player1,
        winner: Winner::NoWinner,
        top_spot: [0; BOARD_WIDTH],
        matrix: [[Counter::NoCounter; BOARD_HEIGHT]; BOARD_WIDTH],
    }
}

fn check_spot(board: &Board, column: isize, row: isize, turn_player: Counter) -> bool {
    // Check a specific spot safely
    if column < 0 || column as usize >= BOARD_WIDTH { return false };
    if row < 0 || row as usize >= BOARD_HEIGHT { return false };
    board.matrix[column as usize][row as usize] == turn_player
}

fn efficient_check(board: &Board, column: usize, row: usize, column_shift: isize, row_shift: isize, turn_player: Counter) -> bool {
    // Carefully search for N in a row to minimise the checks required
    // Note: assumes board[column][row] == turn_player already
    // Trick is just to expand one way til we get to N or hit a non-player counter, then expand the other way until we get to N or don't

    let mut in_a_row_count = 1;

    for i in 1..=(N_IN_A_ROW - 1) {
        if check_spot(&board,
                      column as isize + i as isize * column_shift,
                      row as isize + i as isize * row_shift,
                      turn_player) {
            in_a_row_count += 1;
        } else {
            break;
        }
    }

    if in_a_row_count == N_IN_A_ROW { return true };

    for i in 1..=(N_IN_A_ROW - in_a_row_count) {
        if check_spot(&board, 
                      column as isize - i as isize * column_shift, 
                      row as isize - i as isize * row_shift, 
                      turn_player) {
            in_a_row_count += 1;
            if in_a_row_count == N_IN_A_ROW { return true };
        }
    }

    false
}

impl Board {
    fn winning_move(&self, column: usize, row: usize) -> bool {
        let turn_player = Counter::PlayerCounter(self.turn);
        if row >= N_IN_A_ROW - 1 {
            // vertical check (S)
            let mut vert_in_a_row = 1;
            for i in 1..N_IN_A_ROW {
                if self.matrix[column][row - i] == turn_player {
                    vert_in_a_row += 1;
                    if vert_in_a_row == N_IN_A_ROW { return true }
                } else {
                    break;
                }
            }
        }
        
        // Horizontal check (W-E)
        if efficient_check(&self, column, row, 1, 0, turn_player) { return true };
        // Diagonal check (NW-SE)
        if efficient_check(&self, column, row, 1, -1, turn_player) { return true };
        // Diagonal check (SW-NE)
        if efficient_check(&self, column, row, 1, 1, turn_player) { return true };
        
        false
    }

    fn drawing_move(&self, row: usize) -> bool {
        if row != BOARD_HEIGHT - 1 {
            return false;
        } else {
            return self.top_spot.iter().sum::<usize>() >= BOARD_WIDTH * BOARD_HEIGHT - 1
        }
    }

    pub fn make_move(&mut self, player_move: PlayerMove) -> Result<&Board, MoveError> {
        if self.winner != Winner::NoWinner {
            return Err(MoveError::GameOver);
        }
        if player_move.player != self.turn {
            return Err(MoveError::NotYourTurn);
        }
        if (player_move.column) >= BOARD_WIDTH {
            return Err(MoveError::InvalidColumn);
        }

        let spot = self.top_spot[player_move.column] as usize;

        if spot >= BOARD_HEIGHT {
            return Err(MoveError::ColumnFull);
        }

        if self.winning_move(player_move.column, spot) {
            self.winner = Winner::WinningPlayer(player_move.player);
        } else if self.drawing_move(spot) {
            self.winner = Winner::Draw;
        }

        self.matrix[player_move.column][spot] = Counter::PlayerCounter(player_move.player);
        self.top_spot[player_move.column] += 1;
        self.turn = self.turn.other();

        Ok(self)
    }

}