use std::io::stdin;

use crate::components::{block::Block, board::Board, solver::Solver};

mod components;

fn main() {
    let mut input_iter = stdin().lines().map(|line| line.unwrap().trim().to_string());

    println!("Board Format:");
    println!("  1 2 3 4 - column requirements");
    println!("1 . . . .");
    println!("2 . . . .");
    println!("3 . . . .");
    println!("4 . . . .");
    println!(" \\");
    println!("  row requirements");
    println!();
    println!(". - Empty\n* - Unavailable\n0 - Occupied");

    let mut board_matrix_parse_result: Option<Vec<Vec<char>>> = None;
    while board_matrix_parse_result.is_none() {
        let mut row_strings: Vec<String> = Vec::new();
        loop {
            println!("Please input board content:");
            let row_input = input_iter.next().unwrap();
            if row_input.is_empty() {
                if row_strings.is_empty() {
                    continue;
                }
                break;
            }
            row_strings.push(row_input);
        }
        match Board::parse_board(&row_strings) {
            Ok(result) => {
                board_matrix_parse_result.replace(result);
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    let board_matrix = board_matrix_parse_result.unwrap();

    let mut column_parse_result: Option<Vec<u8>> = None;
    while column_parse_result.is_none() {
        println!("Please input column requirements:");
        let column_input = input_iter.next().unwrap();
        if column_input.is_empty() {
            continue;
        }
        match Board::parse_column_nums(column_input, &board_matrix) {
            Ok(result) => {
                column_parse_result.replace(result);
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    let parsed_column = column_parse_result.unwrap();

    let mut row_parse_result: Option<Vec<u8>> = None;
    while row_parse_result.is_none() {
        println!("Please input row requirements:");
        let row_input = input_iter.next().unwrap();
        if row_input.is_empty() {
            continue;
        }
        match Board::parse_row_nums(row_input, &board_matrix) {
            Ok(result) => {
                row_parse_result.replace(result);
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    let parsed_row = row_parse_result.unwrap();

    let board = Board::new(parsed_row, parsed_column, board_matrix);

    let mut parsed_blocks: Vec<Block> = Vec::new();
    let mut cur_id = 'A' as u8;
    'inp_blk: {
        loop {
            let mut block_parse_result: Option<Block> = None;
            while block_parse_result.is_none() {
                let mut row_strings: Vec<String> = Vec::new();
                println!("Please input pixels for block {}:", cur_id as char);
                let row_input = input_iter.next().unwrap();
                if row_input.is_empty() {
                    if row_strings.is_empty() {
                        continue;
                    }
                    match Block::new(cur_id as char, row_strings) {
                        Ok(result) => {
                            block_parse_result.replace(result);
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                    break;
                }
            }
        }
    }

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
