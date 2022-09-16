extern crate termion;

use std::mem::{size_of_val};
use std::io::{Write, stdout, stdin};

use termion::{color::{self, Color}, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

const CIRCLE: &str = "\u{2B24}";
const PLAYER_1_COLOR: color::Fg<color::Red> = color::Fg(color::Red);
const PLAYER_2_COLOR: color::Fg<color::Blue> = color::Fg(color::Blue);
const DEFAULT_COLOR: color::Fg<color::White> = color::Fg(color::White);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Counter {
    PlayerCounter(Player),
    NoCounter,
}


#[derive(Debug, PartialEq)]
enum Winner {
    WinningPlayer(Player),
    NoWinner,
}

#[derive(Debug)]
struct PlayerMove {
    player: Player,
    column: usize,
}

#[derive(Debug)]
enum MoveError {
    GameOver,
    NotYourTurn,
    InvalidColumn,
    ColumnFull,
}

#[derive(Debug)]
struct Board {
    turn: Player,
    winner: Winner,
    top_spot: [u8; BOARD_WIDTH],
    matrix: [[Counter; BOARD_HEIGHT]; BOARD_WIDTH],
}

fn default_board() -> Board {
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
    // Carefully search for 4 in a row to minimise the checks required
    // Note: assumes board[column][row] == turn_player already
    // Trick is just to expand one way til we get to 4 or hit a non-player counter, then expand the other way until we get 4 total or don't

    let mut in_a_row_count = 1;

    for i in 1..=3 {
        if check_spot(&board,
                      column as isize + i * column_shift,
                      row as isize + i * row_shift,
                      turn_player) {
            in_a_row_count += 1;
        } else {
            break;
        }
    }

    if in_a_row_count == 4 { return true };

    for i in 1..=(4 - in_a_row_count) {
        if check_spot(&board, 
                      column as isize - i * column_shift, 
                      row as isize - i * row_shift, 
                      turn_player) {
            in_a_row_count += 1;
            if in_a_row_count == 4 { return true };
        }
    }

    false
}

impl Board {
    fn winning_move(&self, column: usize, row: usize) -> bool {
        let turn_player = Counter::PlayerCounter(self.turn);
        if row >= 3 {
            // vertical check (S)
            if self.matrix[column][row - 1] == turn_player
            && self.matrix[column][row - 2] == turn_player
            && self.matrix[column][row - 3] == turn_player {
                return true;
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

    fn make_move(&mut self, player_move: PlayerMove) -> Result<&Board, MoveError> {
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
        }
        self.matrix[player_move.column][spot] = Counter::PlayerCounter(player_move.player);
        self.top_spot[player_move.column] += 1;
        self.turn = self.turn.other();

        Ok(self)
    }

    fn display(&self) -> String {
        let mut board_string = String::from("");
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                match self.matrix[x][BOARD_HEIGHT - 1 - y] {
                    Counter::PlayerCounter(Player::Player1) => board_string.push_str(&format!("|{}{}{} ", PLAYER_1_COLOR, CIRCLE, DEFAULT_COLOR)),
                    Counter::PlayerCounter(Player::Player2) => board_string.push_str(&format!("|{}{}{} ", PLAYER_2_COLOR, CIRCLE, DEFAULT_COLOR)),
                    Counter::NoCounter => board_string.push_str("|  "),
                }
            }
            board_string.push_str("|\r\n");
        }
        return board_string
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut board = default_board();
    
    loop {
        let winner_msg = match board.winner {
            Winner::WinningPlayer(Player::Player1) => format!("{}Player 1 Wins!{}", PLAYER_1_COLOR, DEFAULT_COLOR),
            Winner::WinningPlayer(Player::Player2) => format!("{}Player 2 Wins!{}", PLAYER_2_COLOR, DEFAULT_COLOR),
            Winner::NoWinner => String::from(""),
        };

        let stdin = stdin();
        write!(stdout,
            "{}{}\r\n{}{}{}",
            termion::clear::All,
            winner_msg,
            board.display(),
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            // write!(stdout,
            //     "{}{}",
            //     termion::cursor::Goto(1, 1),
            //     termion::clear::CurrentLine)
            //         .unwrap();

            match c.unwrap() {
                Key::Char(x) => {
                    if board.winner == Winner::NoWinner {
                        if let Some(d) = x.to_digit(10) {
                            if d > 0 && d < (BOARD_WIDTH + 1) as u32 {
                                if let Ok(_) = board.make_move(PlayerMove { player: board.turn, column: (d - 1) as usize }) {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        
    }
}
