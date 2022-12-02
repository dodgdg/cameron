use rand::seq::IteratorRandom;
use rand::{self, Rng};
use rand::rngs::ThreadRng;

use crate::board::{Winner, Board, Player, PlayerMove, BOARD_HEIGHT, BOARD_WIDTH};
use crate::movetree::MoveTree;


fn random_move(rng: &mut ThreadRng, board: &mut Board) {
    let random_move = (0..BOARD_WIDTH).filter(| &x | board.top_spot[x] < BOARD_HEIGHT).choose(rng);
    match random_move {
        Some(good_move) => {board.make_move(PlayerMove {player: board.turn, 
                                         column: good_move}).unwrap();},  // HANDLE BETTER
        None => {println!("{}\r\n{:?}\r", board.display(), board.winner);}
    }
}

fn random_game_simulation(board: &mut Board, rng: &mut ThreadRng) -> Winner {
    loop {
        if board.winner != Winner::NoWinner { 
            return board.winner }
        random_move(rng, board);
    }
}

pub fn random_playout(board: &mut Board, playouts: usize, chosen_player: Player) -> usize {
    let mut rng = rand::thread_rng();
    let mut wins = 0;
    
    for _ in 1..=playouts {
        if random_game_simulation(&mut board.clone(), &mut rng) == Winner::WinningPlayer(chosen_player) {
            wins += 1;
        } 
    }
    wins
}
