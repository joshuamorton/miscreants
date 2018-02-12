extern crate float_cmp;
extern crate float_ord;
extern crate probability;

use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use std::marker::PhantomData;
use mcts::float_ord::FloatOrd;
use mcts::probability::distribution::Beta;
use mcts::probability::distribution::Sample;
use mcts::probability::prelude::source;
use mcts::probability::prelude::source::Source;
use mcts::probability::prelude::source::Default;

use game;

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

    pub fn new_seeded(game: G, seed: [u64; 2]) -> MonteCarloAgent<State, Move, G> {
        MonteCarloAgent {
            game: G::new(),
            playouts: MCTSNode::new(),
            phantom: PhantomData,
            source: source::default().seed(seed)
        }
    }

    fn beta_sample(&mut self, alpha: i32, beta: i32) -> f64 {
        let b = Beta::new(alpha as f64, beta as f64, 0.0, 1.0);
        return b.sample(&mut self.source);
    }

    fn pick_move(&mut self, moves: &HashMap<Move, MCTSNode<Move>>) -> Move {
        *moves.iter().max_by_key(|pair| {
            FloatOrd(self.beta_sample(pair.1.successes, pair.1.failures))
        }).expect("I'm not sure why this could fail").0
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

    fn from_data(wins: i32, losses: i32) -> MCTSNode<Move> {
        MCTSNode {
            moves: HashMap::new(),
            successes: wins,
            failures: losses,
        }
    }

}

struct Rollout<State, Move: Copy> {
    moves: Vec<Move>,
    state: State,
}

#[cfg(test)]
mod tests {
    use game::*;
    use mcts::*;
    use mcts::float_cmp::*;
    use std::collections::HashMap;

    struct NullGame {}

    impl Game<i32, i32> for NullGame {
        fn new() -> NullGame {NullGame{} }
        fn get_player(&self) -> Player {Player::One}
        fn get_state(&self) -> &i32 {&1}
        fn set_state(&mut self, state: i32) {}
        fn moves(state: i32, _player: Player) -> Vec<i32> {vec![1,2,3,4]}
        fn evaluate(state: i32, player: Player, action: i32) -> i32 {state + action}
        fn won(state: i32, player: Player) -> bool {false}
    }

    #[test]
    fn test_mca_sample() {
        let g = NullGame::new();
        let mut mca = MonteCarloAgent::new_seeded(g, [1,1]);
        assert!(0.49932.approx_eq_ratio(&mca.beta_sample(1, 1), 0.00001));
    }

    //#[test]
    //fn test_mca_pick() {
        //let g = NullGame::new();
        //let mut mca = MonteCarloAgent::new_seeded(g, [1,1]);
        //let mut moves: HashMap<i32, MCTSNode<i32>> = HashMap::new();
        //moves.insert(1, MCTSNode::from_data(1, 1));
        //moves.insert(2, MCTSNode::from_data(10, 1));
        //moves.insert(3, MCTSNode::from_data(100, 1));
        //moves.insert(4, MCTSNode::from_data(100, 1));
        //assert_eq!(mca.pick_move(&moves), 4);
    //}
}
