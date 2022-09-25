mod board;
use board::Board;

// Let's try the MemoryArena approach, where we will just have a tree of small things including a reference to the board

#[derive(Debug)]
struct Node {
    turn: Player,
    playouts: u32,
    wins: u32,
    board: &Board,
}