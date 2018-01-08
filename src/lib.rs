pub mod game;
use game::Player;

type TicTacToeBoard = Vec<Vec<Option<game::Player>>>;

pub struct TicTacToe {
    board: TicTacToeBoard,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: vec![vec![Option::None; 3]; 3],
        }
    }
}

impl game::Game<TicTacToeBoard, (usize, usize)> for TicTacToe {
    fn get_player(&self) -> Player {
        let positions = self.board.iter().flat_map(|row| row.iter());

        let parity = positions.fold(0, |ct, p| match p {
            &None => ct,
            // I'm mildly amazed that this worked.
            &Some(Player::One) => ct + 1,
            &Some(Player::Two) => ct - 1,
        });
        match parity {
            0 => Player::One,
            1 => Player::Two,
            _ => panic!("Some weird game parity"),
        }
    }

    fn get_state(&self) -> &TicTacToeBoard {
        &self.board
    }

    fn set_state(&mut self, state: TicTacToeBoard) {
        self.board = state;
    }

    fn moves(state: TicTacToeBoard, _player: Player) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if !state[x][y].is_some() {
                    moves.push((x, y));
                }
            }
        }
        return moves;
    }

    fn evaluate(state: TicTacToeBoard, player: Player, action: (usize, usize)) -> TicTacToeBoard {
        if state[action.0][action.1].is_some() {
            panic!("Assigning to an existing position");
        }
        let mut copy = state.to_vec();
        copy[action.0][action.1] = Some(player);
        copy
    }

    fn won(state: TicTacToeBoard, player: Player) -> bool {
        let winning_combos = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        let positions: Vec<&Option<Player>> = state.iter().flat_map(|row| row.iter()).collect();

        // The need for * and & here is a bit unintuitive, but the compiler also
        // basically told me what to do.
        winning_combos
            .iter()
            .any(|c| c.iter().all(|i| positions[*i] == &Some(player)))
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use TicTacToe;
    use TicTacToeBoard;

    const BEGIN_BOARD: [[Option<Player>; 3]; 3] = [
        [None, None, None],
        [None, Some(Player::One), None],
        [None, None, None],
    ];

    const END_BOARD: [[Option<Player>; 3]; 3] = [
        [Some(Player::One), None, Some(Player::Two)],
        [None, Some(Player::One), Some(Player::Two)],
        [None, None, Some(Player::One)],
    ];

    fn convert_board(board: [[Option<Player>; 3]; 3]) -> TicTacToeBoard {
        board.to_vec().iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn test_first_move() {
        let g = TicTacToe::new();
        assert_eq!(Player::One, g.get_player());
    }

    #[test]
    fn test_later_move() {
        let mut g = TicTacToe::new();
        g.set_state(convert_board(BEGIN_BOARD));
        assert_eq!(Player::Two, g.get_player());
    }

    #[test]
    fn test_moves() {
        assert_eq!(
            vec![(0, 1), (1, 0), (2, 0), (2, 1)],
            TicTacToe::moves(convert_board(END_BOARD), Player::One)
        );
    }

    #[test]
    fn test_winner_exists() {
        assert!(TicTacToe::won(convert_board(END_BOARD), Player::One));
    }

    #[test]
    fn test_no_winner_exists() {
        assert!(!TicTacToe::won(convert_board(BEGIN_BOARD), Player::One));
        assert!(!TicTacToe::won(convert_board(END_BOARD), Player::Two));
    }
}
