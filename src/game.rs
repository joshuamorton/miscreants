// Only provide support for two player games, but beyond that should be
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    One,
    Two,
}

/// A Game is an entity that the MCTS algorithm can run over.
///
/// A Game need only be able to provide:
///  - A constructor
///  - An accessor method for the current player
///  - A getter and setter for the game state
///  - A list of valid moves for a player, from a given state
///  - A way to see the impact of a move on the state
///  - A check to see if a player won the game
///
///  There are some potential problems that arise from move being Copy, for
///  example, while this doesn't cause any issues for tic-tac-toe, connect
///  four, go, or chess, a game like Magic the Gathering can have fairly
///  complex actions, even within the context of a turn (which is very, very
///  complex): cast spell X targetting creatures A, B, C, and D by spending
///  mana UUBG is bordering on a complex enough datastructure that it shoud
///  not be copy.
pub trait Game<State, Move: Copy> {

    fn new() -> Self;

    /// Simple getter for the player to move, this should probably eventually
    /// be converted to a trait attribute, even though it can be derived from
    /// the state for most games.
    fn get_player(&self) -> Player;

    /// Simple getter for the game state. This should be a trait.
    fn get_state(&self) -> &State;

    /// Setter for the game state, we wouldn't need this is state was a trait.
    fn set_state(&mut self, state: State);

    /// Method to generate all valid moves from the current position. The
    /// return value should probably be Iterator<Move>, but that isn't
    /// currently expressible.
    /// This might matter for games with incredibly high branching factors
    /// where a generator of moves is better, and you need to reservoir sample
    /// to get the move, but that's a future improvement, and probably a
    /// backwards compatible change.
    fn moves(state: State, player: Player) -> Vec<Move>;

    /// Method to get the resulting game state after a given move.
    fn evaluate(state: State, player: Player, action: Move) -> State;

    ///A check to see if, for a current game state, a player won the game.
    fn won(state: State, player: Player) -> bool;
}
