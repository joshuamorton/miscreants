extern crate probability;

use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use std::marker::PhantomData;
use mcts::probability::distribution::Beta;
use mcts::probability::distribution::Sample;
use mcts::probability::prelude::source;
use mcts::probability::prelude::source::Source;
use mcts::probability::prelude::source::Default;

use game;

fn beta_sample(alpha: f64, beta: f64, src: &mut Default) -> f64 {
    let b = Beta::new(alpha, beta, 0.0, 1.0);
    return b.sample(src);
}

pub struct MonteCarloAgent<State, Move: Copy + Hash + Eq, G: game::Game<State, Move>> {
    game: G,
    playouts: MCTSNode<Move>,
    phantom: PhantomData<State>,
    source: Default,
}

impl<State, Move: Copy + Hash + Eq, G: game::Game<State, Move>> MonteCarloAgent<State, Move, G> {
    pub fn new(game: G) -> MonteCarloAgent<State, Move, G> {
        MonteCarloAgent {
            game: G::new(),
            playouts: MCTSNode::new(),
            phantom: PhantomData,
            source: source::default(),
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


#[cfg(test)]
mod tests {
}
