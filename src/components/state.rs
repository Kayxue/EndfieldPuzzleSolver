use crate::components::board::Board;

pub struct State {
    block_used: u16,
    current_board: Board,
}

impl State {
    pub fn new(block_count: u8, board: Board) -> State {
        State {
            block_used: 0,
            current_board: board,
        }
    }

    pub fn next_state(
        &self,
        id: &char,
        block: &Vec<Vec<char>>,
        position: (usize, usize),
    ) -> Option<State> {
        let block_used = *id as u8 - 'A' as u8;

        if self.block_used & (1 << block_used) != 0 {
            return None;
        }

        let board_next_state = self.current_board.place_block(block, position);
        if board_next_state.is_none() {
            return None;
        }

        let block_used_next_state = self.block_used | (1 << block_used);

        Some(State {
            block_used: block_used_next_state,
            current_board: board_next_state.unwrap(),
        })
    }

    pub fn is_finish_state(&self, totalBlocks: u8) -> bool {
        for b in 0..totalBlocks {
            if (1 << b) & self.block_used == 0 {
                return false;
            }
        }
        return true;
    }

    pub fn is_block_used(&self, id: &char) -> bool {
        let block_used = *id as u8 - 'A' as u8;
        self.block_used & (1 << (block_used)) != 0
    }

    pub fn get_board(&self) -> &Board {
        &self.current_board
    }
}
