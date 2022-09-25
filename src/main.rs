extern crate termion;

use std::mem::{size_of_val};
use std::io::{Write, stdout, stdin};

use termion::{color::{self, Color}, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const CIRCLE: &str = "\u{2B24}";
const PLAYER_1_COLOR: color::Fg<color::Red> = color::Fg(color::Red);
const PLAYER_2_COLOR: color::Fg<color::Blue> = color::Fg(color::Blue);
const DEFAULT_COLOR: color::Fg<color::White> = color::Fg(color::White);

mod board;
use board::{Player, PlayerMove, Counter, Board, Winner, BOARD_WIDTH, BOARD_HEIGHT};

impl Board {
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

    let mut board = board::default_board();
    
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
