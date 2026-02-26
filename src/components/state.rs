use crate::components::board::Board;

pub struct State {
    block_used: u16,
    block_left: u8,
    current_board: Board,
}

impl State {
    pub fn new(block_count: u8, board: Board) -> State {
        State {
            block_used: 0,
            block_left: block_count,
            current_board: board,
        }
    }

    pub fn next_state(
        &self,
        id: char,
        block: &Vec<Vec<char>>,
        position: (usize, usize),
    ) -> Option<State> {
        let board_next_state = self.current_board.place_block(block, position);
        if board_next_state.is_none() {
            return None;
        }

        let bit_to_change = id as u8 - 'A' as u8;
        let block_used_next_state = self.block_used | (1 << (15 - bit_to_change));
        let new_block_left = self.block_left - 1;

        Some(State {
            block_used: block_used_next_state,
            block_left: new_block_left,
            current_board: board_next_state.unwrap(),
        })
    }
}
