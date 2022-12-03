extern crate termion;

use std::fmt::format;
use std::mem::{size_of_val};
use std::io::{Write, stdout, stdin};
use std::time::{Duration, Instant};

use termion::{color::{self, Color}, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const CIRCLE: &str = "\u{2B24}";
const PLAYER_1_COLOR: color::Fg<color::Red> = color::Fg(color::Red);
const PLAYER_2_COLOR: color::Fg<color::Blue> = color::Fg(color::Blue);
const DEFAULT_COLOR: color::Fg<color::White> = color::Fg(color::White);

const COMPUTER_PLAYER: Player = Player::Player2;

mod board;
use board::{Player, PlayerMove, Counter, Board, Winner, BOARD_WIDTH, BOARD_HEIGHT};
mod movetree;
use movetree::{MoveTree, default_move_tree};
mod montecarlo;


fn think_for_seconds(brain: &mut MoveTree, board: &mut Board, seconds: f32) {
    let interval = Duration::from_millis((seconds * 1000.0) as u64);
    let stop_time = Instant::now() + interval;

    loop {
        brain.think(&mut board.clone());
        
        if Instant::now() > stop_time {
            break;
        }
    }
}


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
    
    let mut brain = default_move_tree();
    
    // think_for_seconds(&mut brain, &mut board, 200.0);

    let mut lastmove = 0;

    loop {
        let winner_msg = match board.winner {
            Winner::WinningPlayer(Player::Player1) => format!("{}Player 1 Wins!{}", PLAYER_1_COLOR, DEFAULT_COLOR),
            Winner::WinningPlayer(Player::Player2) => format!("{}Player 2 Wins!{}", PLAYER_2_COLOR, DEFAULT_COLOR),
            Winner::Draw => String::from("Game Drawn!"),
            Winner::NoWinner => format!("{} turn", if board.turn == COMPUTER_PLAYER {"My"} else {"Your"}),
        };

        let stdin = stdin();
        write!(stdout,
            "{}{}\r\n{}\r  1  2  3  4  5  6  7{}{}",
            termion::clear::All,
            winner_msg,
            board.display(),
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
        )
        .unwrap();
        stdout.flush().unwrap();

        if board.winner == Winner::NoWinner {
            if board.turn == COMPUTER_PLAYER {

                think_for_seconds(&mut brain, &mut board, 2.0);

                lastmove = brain.best_move(&board);
                
                board.make_move(PlayerMove {player: board.turn, column: lastmove}).unwrap();
                brain.traverse_root(lastmove);
            } else {
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char(x) => {
                            if let Some(d) = x.to_digit(10) {
                                if d > 0 && d < (BOARD_WIDTH + 1) as u32 {
                                    if let Ok(_) = board.make_move(PlayerMove { player: board.turn, column: (d - 1) as usize }) {
                                        brain.traverse_root((d - 1) as usize);
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            break;
        }
    }
    let winner_msg = match board.winner {
        Winner::WinningPlayer(Player::Player1) => format!("{}Player 1 Wins!{}", PLAYER_1_COLOR, DEFAULT_COLOR),
        Winner::WinningPlayer(Player::Player2) => format!("{}Player 2 Wins!{}", PLAYER_2_COLOR, DEFAULT_COLOR),
        Winner::Draw => String::from("Game Drawn!"),
        Winner::NoWinner => format!("{} turn", if board.turn == COMPUTER_PLAYER {"My"} else {"Your"}),
    };

    write!(stdout,
        "{}{}\r\n{}\r  1  2  3  4  5  6  7",
        termion::clear::All,
        winner_msg,
        board.display(),
    )
    .unwrap();
    stdout.flush().unwrap();

}
