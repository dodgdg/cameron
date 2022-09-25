use rand;

use crate::board::{Winner, Board};
use crate::movetree::MoveTree;


fn random_move(board: &mut Board) {

}

pub fn random_game_simulation(board: &mut Board) -> Winner {
    loop {
        random_move(board);
        // NOTE need to account for no moves left!!! (draw)
        if board.winner != Winner::NoWinner { return board.winner }
    }
}
