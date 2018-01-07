// Only provide support for two player games, but beyond that should be
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    One,
    Two,
}

/// A Game is an entity that the MCTS algorithm can run over.
///
/// A Game need only be able to provide:
///  - An accessor method for the current player
///  - A getter and setter for the game state
///  - A list of valid moves for a player, from a given state
///  - A way to see the impact of a move on the state
///  - A check to see if a player won the game
pub trait Game<State, Move: Copy> {
    fn get_player(&self) -> Player;
    fn get_state(&self) -> &State;
    fn set_state(&mut self, state: State);
    fn moves(state: State, player: Player) -> Vec<Move>;
    fn evaluate(state: State, player: Player, action: Move) -> State;
    fn won(state: State, player: Player) -> bool;
}
