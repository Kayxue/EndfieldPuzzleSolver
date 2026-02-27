use crate::components::{block::Block, board::Board, solver::Solver};

mod components;

fn main() {
    let board = vec![
            ".....".to_owned(),
            ".....".to_owned(),
            ".....".to_owned(),
            "...*.".to_owned(),
            "....*".to_owned(),
        ];
        let column_input = "4 5 3 2 1".to_owned();
        let row_input = "4 5 3 2 1".to_owned();

        let board_instance = Board::parse_board(&board).unwrap();
        let parsed_column = Board::parse_column_nums(column_input, &board).unwrap();
        let parsed_row = Board::parse_row_nums(row_input, &board).unwrap();

        let board = Board::new(parsed_row, parsed_column, board_instance);

        let blocks = vec![
            vec!["000".to_owned(), "0".to_owned()],
            vec!["000".to_owned(), ".0".to_owned()],
            vec!["000".to_owned(), ".0".to_owned()],
            vec!["00".to_owned(), "0".to_owned()],
        ];

        let parsed_blocks: Vec<Block> = blocks
            .iter()
            .enumerate()
            .map(|(index, e)| Block::new(('A' as u8 + index as u8) as char, e.to_vec()).unwrap())
            .collect();

        let solver = Solver::new(parsed_blocks, board);

        solver.solve();

        for (i, solution) in solver.get_solution_states().iter().enumerate() {
            println!("Solution {}", i + 1);
            for r in solution.get_board().get_contents() {
                println!("{}", String::from_iter(r));
            }
            println!("----------------");
        }
}
