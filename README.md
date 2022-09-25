# cameron
A faster connect 4 AI in Rust.

### NOTES:
1) Ideally the set of states is a digraph, to account for different move orders, AND I think the move-selection algorithm can work with this (Connect 4 digraph has no cycles because the number of counters always increases), BUT I will stick with a tree for now, because it's simpler, and also I'd like to compare the performance of both approaches anyway.
2) I want the move tree to be present for the whole game, and (with multithreading) be constantly updated. As players make their moves, we can 'zoom' on the tree and delete un-needed branches.
3) The hope is to have a Monte Carlo Tree Search implemented, with potentially some intelligent AB-minimax at the ends of the tree to avoid endlessly simulating / to give 100% certainty to some lines.
4) I really love the idea of creating endgame tables for Connect 4 but I haven't got a clear idea of what these would look like yet. WIP.
5) One thing I want to make use of is multithreading - initially I considered having several threads active on the move tree at once, but since I anticipate simulation to take the most time, better to have one actor and then delegate simulation to many threads. As mentioned above I also want the move tree to be constantly updated by another thread, even while the other player is thinking.
6) Also of high importance is diagnostics/stats of the algorithm as it runs, to see what it's thinking about. Best move sequences can be displayed on the board itself which would look pretty cool I think.

### TODO:
- ~~Implement board and playing~~
- ~~Create move tree~~
- ~~Separate into different files / cleanup~~
- Implement MCTS on the move tree
- Add multithreading
- Add diagnostics / stats
- Create move digraph
- Extras (clever AB-minimax, endgame tables, custom move-selection heuristics)