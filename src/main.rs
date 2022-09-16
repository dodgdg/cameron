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

impl Board {
//     fn winning_move(&self, column: usize, row: usize) -> bool {
//         let turn_player = Counter::PlayerCounter(self.turn);
// //  SHIT you forgot that we could be completing the line from the middle also
//         if row >= 3 {
//             // vertical check (S)
//             if self.matrix[column][row - 1] == turn_player
//             && self.matrix[column][row - 2] == turn_player
//             && self.matrix[column][row - 3] == turn_player {
//                 return true;
//             }
//             // diagonal check SE
//             if column <= BOARD_WIDTH - 4
//             && self.matrix[column + 1][row - 1] == turn_player
//             && self.matrix[column][row - 2] == turn_player
//             && self.matrix[column][row - 3] == turn_player {
//                 return true;
//             }
//             // diagonal check SW
//             if column >= 3
//             && self.matrix[column][row - 1] == turn_player
//             && self.matrix[column][row - 2] == turn_player
//             && self.matrix[column][row - 3] == turn_player {
//                 return true;
//             }

//         if row <= BOARD_HEIGHT - 4 {
//             // no vertical check (N)

//             // diagonal check NE
//             if column <= BOARD_WIDTH - 4
//             && self.matrix[column + 1][row - 1] == turn_player
//             && self.matrix[column][row - 2] == turn_player
//             && self.matrix[column][row - 3] == turn_player {
//                 return true;
//             }
//             // diagonal check NW
//             if column >= 3
//             && self.matrix[column][row - 1] == turn_player
//             && self.matrix[column][row - 2] == turn_player
//             && self.matrix[column][row - 3] == turn_player {
//                 return true;
//             }

//         return false;
//     }

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
        
        // if self.winning_move(player_move.column, spot) {
            // self.winner = Winner::WinningPlayer(player_move.player);
        // }
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
        let stdin = stdin();
        write!(stdout,
            "{}\r\n{}{}{}",
            termion::clear::All,
            board.display(),
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            write!(stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine)
                    .unwrap();

            match c.unwrap() {
                Key::Char(x) => {
                    if let Some(d) = x.to_digit(10) {
                        if d > 0 && d < (BOARD_WIDTH + 1) as u32 {
                            board.make_move(PlayerMove { player: board.turn, column: (d - 1) as usize }).unwrap();
                            break;
                        }
                    }
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}
