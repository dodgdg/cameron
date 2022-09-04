extern crate termion;

use termion::{color::{self, Color}, style};
use std::mem::{size_of_val};

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

const CIRCLE: &str = "\u{2B24}";
const PLAYER_1_COLOR_COLOR: color::Fg<color::Red> = color::Fg(color::Red);
const PLAYER_2_COLOR: color::Fg<color::Blue> = color::Fg(color::Blue);

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

#[derive(Clone, Copy, Debug)]
enum Counter {
    PlayerCounter(Player),
    NoCounter,
}

#[derive(Debug)]
struct PlayerMove {
    player: Player,
    column: usize,
}

#[derive(Debug)]
enum MoveError {
    NotYourTurn,
    InvalidColumn,
    ColumnFull,
}

#[derive(Debug)]
struct Board {
    turn: Player,
    top_spot: [u8; BOARD_WIDTH],
    matrix: [[Counter; BOARD_HEIGHT]; BOARD_WIDTH],
}

fn default_board() -> Board {
    Board {
        turn: Player::Player1,
        top_spot: [0; BOARD_WIDTH],
        matrix: [[Counter::NoCounter; BOARD_HEIGHT]; BOARD_WIDTH],
    }
}

impl Board {
    fn make_move(&mut self, player_move: PlayerMove) -> Result<&Board, MoveError> {
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
        
        self.matrix[player_move.column][spot] = Counter::PlayerCounter(player_move.player);
        self.top_spot[player_move.column] += 1;
        self.turn = self.turn.other();
        Ok(self)
    }
}


fn main() {
    let mut board = default_board();
    println!("{:?}", board.make_move(PlayerMove { player: Player::Player1, column: 4 }).unwrap());
    println!("{:?}", board.make_move(PlayerMove { player: Player::Player2, column: 4 }).unwrap());

    println!("size of board: {}", size_of_val(&board));
    println!("size of matrix: {}", size_of_val(&board.matrix));
    println!("size of turn: {}", size_of_val(&board.turn));
    println!("size of top spot: {}", size_of_val(&board.top_spot));

    let x: u8 = 4;
    let y = Counter::PlayerCounter(Player::Player1);

    println!("size of u8: {}", size_of_val(&x));
    println!("size of counter: {}", size_of_val(&y));

    println!("Meh");
}
