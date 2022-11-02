use rand::seq::IteratorRandom;
use rand::{self, Rng};
use rand::rngs::ThreadRng;

use crate::board::{Winner, Board, Player, PlayerMove, BOARD_HEIGHT, BOARD_WIDTH};
use crate::movetree::MoveTree;


fn random_move(rng: &mut ThreadRng, board: &mut Board) {
    let random_move = board.top_spot.iter().filter(| &&x | x < BOARD_HEIGHT).choose(rng).unwrap();
    board.make_move(PlayerMove {player: board.turn.other(), 
                                column: *random_move});
}

fn random_game_simulation(board: &mut Board, rng: &mut ThreadRng) -> Winner {
    loop {
        random_move(rng, board);
        if board.winner != Winner::NoWinner { return board.winner }
    }
}

pub fn random_playout(board: &mut Board, playouts: usize, chosen_player: Player) -> usize {
    let mut rng = rand::thread_rng();
    let mut wins = 0;
    
    for _ in 1..playouts {
        match random_game_simulation(&mut board.clone(), &mut rng) {
            Winner::WinningPlayer(chosen_player) => wins += 1,
            _ => ()
        }
    };
    
    wins
}
