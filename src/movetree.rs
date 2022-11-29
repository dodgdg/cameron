use slab::Slab;

use crate::board::{Player, PlayerMove, Board, BOARD_WIDTH, BOARD_HEIGHT, Winner};
use crate::montecarlo::random_playout;

const EXPLORE: f32 = 10.0;

// I'm thinking that since we already edit the state of Board loads during the simulation phase,
// there's no harm in just storing the moves we make, and updating a fresh board as we traverse the tree.
// Let's see.

// struct NodePool {
//     nodes: Vec<Node>,
// }

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Node {
    parent: Option<usize>,
    children: [Option<usize>; BOARD_WIDTH],
    data: GameState,
    score: f32,
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
            score: 0.,
        }
    );

    MoveTree {
        nodes: slab,
        root: root_ix,
    }
}

fn get_score(wins: usize, playouts: usize, parent_playouts: usize) -> f32 {
    wins as f32 / (0.01 + playouts as f32) as f32 + (EXPLORE * ((playouts + parent_playouts) as f32).ln() / (0.01 + playouts as f32)).sqrt()  // NEED TO IMPROVE (NO 0.01 + ...)
}

fn choose_max_index<T: std::cmp::PartialOrd>(vec: &Vec<T>) -> usize {
    vec.iter().enumerate().reduce(| max_so_far, other | 
                                  if other.1 > max_so_far.1 {other} else {max_so_far}).unwrap().0
}

impl MoveTree {
    pub fn traverse_root(&mut self, move_index: usize) {
        self.root = match self.nodes[self.root].children[move_index] {
            Some(ix) => ix,
            None => self.add_node(self.root, move_index, 0, 0)
        };
        self.nodes[self.root].parent = None;
    }

    fn update_node(&mut self, mut node: Node, wins: usize, playouts: usize) -> Node {
        node.data.playouts += playouts;
        node.data.wins += wins;
        if let Some(parent) = node.parent {
            node.score = get_score(node.data.wins, node.data.playouts, self.nodes[parent].data.playouts);
        }
        node
    }

    fn update_playout(&mut self, node_index: usize, wins: usize, playouts: usize) {
        let mut current_index = node_index;
        loop {
            self.nodes[current_index] = self.update_node(self.nodes[current_index], wins, playouts);
            if let Some(parent) = self.nodes[current_index].parent {
                current_index = parent;
            } else {
                break;
            }
        }
    }

    pub fn add_node(&mut self, node_index: usize, column: usize, wins: usize, playouts: usize) -> usize {
        let ix = self.nodes.insert(
            Node {
                parent: Some(node_index),
                children: [None; BOARD_WIDTH],
                data: GameState { 
                    turn: self.nodes[node_index].data.turn.other(),
                    playouts: playouts,
                    wins: wins,
                },
                score: get_score(wins, playouts, playouts + self.nodes[node_index].data.playouts),
            });

        self.nodes[node_index].children[column] = Some(ix);
        ix
    }

    pub fn add_playout(&mut self, node_index: usize, column: usize, wins: usize, playouts: usize) {
        // add the node
        let new_ix = self.add_node(node_index, column, wins, playouts);
        // update the tree
        self.update_playout(new_ix, wins, playouts);
    }

    pub fn best_move(&self, current_board: &Board) -> usize {
        let root_node = self.nodes[self.root];
        let available_moves = (0..BOARD_WIDTH).filter(
                                                                | &x | current_board.top_spot[x] < BOARD_HEIGHT).collect::<Vec<_>>();
        let scores = available_moves.iter().map(| &mov | {
                                                                if let Some(ix) = root_node.children[mov] {
                                                                    self.nodes[ix].data.wins as f32 
                                                                    / (0.01 + self.nodes[ix].data.playouts as f32) // NEED TO IMPROVE (NO 0.01 + ...)
                                                                } else {
                                                                    0.0
                                                                }}).collect::<Vec<_>>();
        let ix = choose_max_index(&scores);
        println!("score: {}\r", scores[ix]);
        available_moves[ix]
    }

    // Main function
    pub fn think(&mut self, current_board: &mut Board) {
        let mut next_move;
        let mut current_ix = self.root;
        let mut available_moves;

        // println!("root:{}\r", current_ix);

        loop {
            // make move
            available_moves = (0..BOARD_WIDTH).filter(| &x | current_board.top_spot[x] < BOARD_HEIGHT).collect::<Vec<_>>();

            // println!("current index: {:?}\r", current_ix);
            // println!("moves: {:?}\r", available_moves);
            // println!("top: {:?}\r", current_board.top_spot);

            let scores = available_moves.iter().map(| &mov | {
                if let Some(ix) = self.nodes[current_ix].children[mov] {
                    self.nodes[ix].score
                } else {
                    get_score(0, 0, self.nodes[current_ix].data.playouts)
                }}).collect::<Vec<_>>();
             
            // println!("scores: {:?}\r", scores);

            next_move = available_moves[choose_max_index(&scores)];
            // println!("next move: {}\r", next_move);

            current_board.make_move(PlayerMove { player: current_board.turn, 
                column: next_move }).unwrap();  // HANDLE BETTER

            // println!("{}\r", current_board.display());
            
            if current_board.winner != Winner::NoWinner {
                // println!("winner\r");
                let wins = {if current_board.winner == Winner::WinningPlayer(Player::Player2) {1} else {0}};
                self.add_playout(current_ix, next_move, wins, 1);
                return;
            }

            if let Some(ix) = self.nodes[current_ix].children[next_move] {
                current_ix = ix;
            } else {
                break;
            }
        }
            
        // println!("stop\r");
        // println!("{}\r", current_board.display());

        // add a new random playout

        let playouts = 100;
        let playout_wins = random_playout(current_board, playouts, Player::Player2);
        let wins = if (playout_wins as f32 / playouts as f32) > 0.5 {1} else {0};
        self.add_playout(current_ix, next_move, wins, 1);


        // println!("added playout");
    }

    // Main function
    pub fn think_out_loud(&mut self, current_board: &mut Board) {
        let mut next_move;
        let mut current_ix = self.root;
        let mut available_moves;
        
        println!("root:{}\r", current_ix);

        loop {
            // make move
            available_moves = (0..BOARD_WIDTH).filter(| &x | current_board.top_spot[x] < BOARD_HEIGHT).collect::<Vec<_>>();

            println!("current index: {:?}\r", current_ix);
            println!("moves: {:?}\r", available_moves);
            println!("top: {:?}\r", current_board.top_spot);

            let scores = available_moves.iter().map(| &mov | {
                if let Some(ix) = self.nodes[current_ix].children[mov] {
                    self.nodes[ix].score
                } else {
                    get_score(0, 0, self.nodes[current_ix].data.playouts)
                }}).collect::<Vec<_>>();
             
            println!("scores: {:?}\r", scores);

            next_move = available_moves[choose_max_index(&scores)];
            println!("next move: {}\r", next_move);

            current_board.make_move(PlayerMove { player: current_board.turn, 
                column: next_move }).unwrap();  // HANDLE BETTER

            println!("{}\r", current_board.display());
            
            if current_board.winner != Winner::NoWinner {
                println!("winner\r");
                let wins = {if current_board.winner == Winner::WinningPlayer(Player::Player2) {1} else {0}};
                self.add_playout(current_ix, next_move, wins, 1);
                return;
            }

            if let Some(ix) = self.nodes[current_ix].children[next_move] {
                current_ix = ix;
            } else {
                break;
            }
        }
            
        println!("stop\r");
        println!("{}\r", current_board.display());

        // add a new random playout

        let playouts = 100;
        let playout_wins = random_playout(current_board, playouts, Player::Player2);  // TODO WE NEED TO NEGAMAX YOU IDIOT!

        println!("{} wins\r", playout_wins);
        let wins = if (playout_wins as f32 / playouts as f32) > 0.5 {1} else {0};
        self.add_playout(current_ix, next_move, wins, 1);


        // println!("added playout");
    }
}
