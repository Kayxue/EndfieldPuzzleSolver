use std::{
    cell::{Ref, RefCell},
    collections::{HashSet, VecDeque},
};

use smallvec::SmallVec;

use crate::{
    components::{board::Board, state::State},
    types::BlockVec,
};

#[derive(Debug)]
pub struct Solver {
    blocks: BlockVec,
    states: RefCell<VecDeque<State>>,
    solutions: RefCell<HashSet<State>>,
}

impl Solver {
    pub fn new(blocks: BlockVec, initial_board: Board) -> Solver {
        let initial_state = State::new(initial_board);
        let initial_states = VecDeque::from([initial_state]);
        let mut blocks_sorted = SmallVec::from(blocks);
        blocks_sorted.sort_by(|a, b| {
            b.get_filled_pixels_count()
                .cmp(&a.get_filled_pixels_count())
        });
        Solver {
            blocks: blocks_sorted,
            states: RefCell::from(initial_states),
            solutions: RefCell::from(HashSet::new()),
        }
    }

    pub fn solve(&self) {
        if !self.solvable() {
            return;
        }

        let mut state = self.states.borrow_mut();
        let mut solution_states = self.solutions.borrow_mut();
        let mut already_visited: HashSet<State> = HashSet::new();

        let (board_height, board_width) = state.front().unwrap().get_board().get_size();

        while !state.is_empty() {
            let cur_state = state.pop_front().unwrap();

            if cur_state.is_finish_state(self.blocks.len() as u8) {
                solution_states.insert(cur_state);
                continue;
            }

            if already_visited.contains(&cur_state) {
                continue;
            }

            for block_instance in &self.blocks {
                if cur_state.is_block_used(block_instance.get_id()) {
                    continue;
                }
                for block_state in block_instance.get_block_rotate_state() {
                    for r_index in 0..board_height {
                        if r_index + (block_state.len() - 1) as u8 >= board_height {
                            break;
                        }
                        for c_index in 0..board_width {
                            if c_index + (block_state[0].len() - 1) as u8 >= board_width {
                                break;
                            }

                            let next_state_result = cur_state.next_state(
                                block_instance.get_id(),
                                block_instance.get_color(),
                                block_state,
                                (r_index as usize, c_index as usize),
                            );

                            if let Some(next_state) = next_state_result {
                                state.push_back(next_state);
                            }
                        }
                    }
                }
            }

            already_visited.insert(cur_state);
        }
    }

    fn solvable(&self) -> bool {
        let state = self.states.borrow();

        let first_state_board = state.front().unwrap().get_board();
        let first_state_board_content = first_state_board.get_contents();

        // Check color requirement rows for all block colors exist
        if self
            .blocks
            .iter()
            .any(|b| *(b.get_color()) + 1 > first_state_board.get_colors_count())
        {
            return false;
        }
        
        // Check empty cell is enough
        let block_pixel_total: u8 = self
            .blocks
            .iter()
            .map(|e| e.get_filled_pixels_count())
            .sum();
        let board_empty_pixels_total: u8 = first_state_board_content
            .iter()
            .map(|r| r.iter().filter(|&&c| c == '.').count())
            .sum::<usize>() as u8;

        block_pixel_total <= board_empty_pixels_total
    }

    pub fn get_solution_states(&self) -> Ref<'_, HashSet<State>> {
        self.solutions.borrow()
    }
}
