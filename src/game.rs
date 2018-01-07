
// Only provide support for two player games, but beyond that should be 
pub enum Player {
    One,
    Two,
}

/// A Game is an entity that the MCTS algorithm can run over.
///
/// A Game need only be able to provide:
///  - An accessor method for the current player
///  - An accessor method for the game state
///  - A list of valid moves for a player, from a given state
///  - A way to see the state after a move completes
///  - A check to see if a player won the game
pub trait Game<State, Move> {
    fn get_player(&self) -> Player;
    fn get_state(&self) -> State;
    fn next_move(&self, player: Player) -> Box<Vec<Move>>;
    fn evaluate(&self, action: Move) -> State;
    fn won(&self, player: Player) -> bool;
}

