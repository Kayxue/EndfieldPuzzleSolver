use crate::{components::board::Board, types::BlockContent};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct State {
    block_used: u16,
    current_board: Board,
}

impl State {
    pub fn new(board: Board) -> State {
        State {
            block_used: 0,
            current_board: board,
        }
    }

    pub fn next_state(
        &self,
        id: &char,
        block: &BlockContent,
        position: (usize, usize),
    ) -> Option<State> {
        let block_used = *id as u8 - 'A' as u8;

        if self.block_used & (1 << block_used) != 0 {
            return None;
        }

        let board_next_state = self.current_board.place_block(id, block, position);
        if board_next_state.is_none() {
            return None;
        }

        let block_used_next_state = self.block_used | (1 << block_used);

        Some(State {
            block_used: block_used_next_state,
            current_board: board_next_state.unwrap(),
        })
    }

    pub fn is_finish_state(&self, total_blocks: u8) -> bool {
        for b in 0..total_blocks {
            if (1 << b) & self.block_used == 0 {
                return false;
            }
        }
        self.current_board.is_row_all_zero() && self.current_board.is_column_all_zero()
    }

    pub fn is_block_used(&self, id: &char) -> bool {
        let block_used = *id as u8 - 'A' as u8;
        self.block_used & (1 << (block_used)) != 0
    }

    pub fn get_board(&self) -> &Board {
        &self.current_board
    }
}
