use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use std::marker::PhantomData;

use game;

pub struct MonteCarloAgent<State, Move: Copy + Hash + Eq, G: game::Game<State, Move>> {
    game: G,
    playouts: MCTSNode<Move>,
    phantom: PhantomData<State>,
}

impl<State, Move: Copy + Hash + Eq, G: game::Game<State, Move>> MonteCarloAgent<State, Move, G> {
    pub fn new(game: G) -> MonteCarloAgent<State, Move, G> {
        MonteCarloAgent {
            game: G::new(),
            playouts: MCTSNode::new(),
            phantom: PhantomData,
        }
    }
}

struct MCTSNode<Move: Hash + Eq> {
    moves: HashMap<Move, MCTSNode<Move>>,
    successes: i32,
    failures: i32,
}

impl<Move: Hash + Eq> MCTSNode<Move> {
    fn new() -> MCTSNode<Move> {
        MCTSNode {
            moves: HashMap::new(),
            successes: 0,
            failures: 0,
        }
    }
}

struct Rollout<State, Move: Copy> {
    moves: Vec<Move>,
    state: State,
}
