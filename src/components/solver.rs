use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
};

use crate::components::{block::Block, board::Board, state::State};

pub struct Solver {
    blocks: Vec<Block>,
    states: RefCell<VecDeque<State>>,
    solutions: RefCell<Vec<State>>,
}

impl Solver {
    pub fn new(blocks: Vec<Block>, initial_board: Board) -> Solver {
        let initial_state = State::new(blocks.len() as u8, initial_board);
        let initial_states = VecDeque::from([initial_state]);
        Solver {
            blocks,
            states: RefCell::from(initial_states),
            solutions: RefCell::from(Vec::new()),
        }
    }

    //TODO: Finish the function
    pub fn solve() {
        todo!()
    }

    pub fn get_solution_states(&self) -> Ref<'_, Vec<State>> {
        self.solutions.borrow()
    }
}
