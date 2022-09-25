use slab::Slab;

use crate::board::{Player, BOARD_WIDTH};


// I'm thinking that since we already edit the state of Board loads during the simulation phase,
// there's no harm in just storing the moves we make, and updating a fresh board as we traverse the tree.
// Let's see.

// struct NodePool {
//     nodes: Vec<Node>,
// }

#[derive(Debug)]
pub struct GameState {
    pub turn: Player,
    pub playouts: usize,
    pub wins: usize,
}

pub fn default_game_state() -> GameState {
    GameState {
        turn: Player::Player1,
        playouts: 0,
        wins: 0,
    }
}

#[derive(Debug)]
pub struct Node {
    parent: Option<usize>,
    children: [Option<usize>; BOARD_WIDTH],
    data: GameState,
}

#[derive(Debug)]
pub struct MoveTree {
    pub nodes: Slab<Node>,
    pub root: usize,
}

pub fn default_move_tree() -> MoveTree {
    let mut slab = Slab::new();
    let root_ix = slab.insert(
        Node {
            parent: None,
            children: [None; BOARD_WIDTH],
            data: default_game_state(),
        }
    );

    MoveTree {
        nodes: slab,
        root: root_ix,
    }
}

fn update_node(mut game_state: &mut GameState, win: bool) {
    game_state.playouts += 1;
    if win {
        game_state.wins += 1;
    }
}

impl MoveTree {
    fn update_playout(&mut self, node_index: usize, win: bool) {
        let mut current_index = node_index;
        loop {
            let mut node = &mut self.nodes[current_index];
            update_node(&mut node.data, win);
            if let Some(parent) = node.parent {
                current_index = parent;
            } else {
                break;
            }
        }
    }

    pub fn add_playout(&mut self, node_index: usize, row: usize, win: bool) {
        // add the node
        let ix = self.nodes.insert(
            Node {
                parent: Some(node_index),
                children: [None; BOARD_WIDTH],
                data: GameState { 
                    turn: self.nodes[node_index].data.turn.other(),
                    playouts: 1,
                    wins: {if win {1} else {0}},
                },
            });

        self.nodes[node_index].children[row - 1] = Some(ix);
        self.update_playout(node_index, win);
    }

    pub fn traverse(&self, node_index: usize, row: usize) -> Option<usize>{
        self.nodes[node_index].children[row - 1]
    }

}
