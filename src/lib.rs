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
                if state[x][y].is_some() {
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
        false
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    use TicTacToe;

    #[test]
    fn test_first_move() {
        let g = TicTacToe::new();
        assert_eq!(Player::One, g.get_player());
    }

    #[test]
    fn test_later_move() {
        let mut g = TicTacToe::new();
        g.set_state(vec![
            vec![None, None, None],
            vec![None, Some(Player::One), None],
            vec![None, None, None],
        ]);
        assert_eq!(Player::Two, g.get_player());
    }

}
