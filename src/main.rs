use std::io::stdin;

#[cfg(target_os = "windows")]
use std::process::Command;

use smallvec::SmallVec;

use crate::{
    components::{block::Block, board::Board, solver::Solver},
    types::{BlockVec, BoardContent, RequirementNums, StringInputs},
};

mod components;
mod types;

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

    let mut board_matrix_parse_result: Option<BoardContent> = None;
    'in_brd: while board_matrix_parse_result.is_none() {
        let mut row_strings: StringInputs = SmallVec::new();
        println!("Please enter the board content:");
        loop {
            let row_input = input_iter.next().unwrap();
            if row_input.is_empty() {
                if row_strings.is_empty() {
                    continue 'in_brd;
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

    let mut column_parse_result: Option<RequirementNums> = None;
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

    let mut row_parse_result: Option<RequirementNums> = None;
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

    let mut parsed_blocks: BlockVec = SmallVec::new();
    let mut cur_id = 'A' as u8;
    'in_blk: loop {
        let mut row_strings: StringInputs = SmallVec::new();
        println!("Please input pixels for block {}:", cur_id as char);
        loop {
            let row_input = input_iter.next().unwrap();
            if row_input.is_empty() {
                if row_strings.is_empty() && parsed_blocks.is_empty() {
                    continue 'in_blk;
                }
                break;
            }
            row_strings.push(row_input);
        }
        if row_strings.is_empty() {
            break 'in_blk;
        }
        match Block::new(cur_id as char, row_strings) {
            Ok(result) => {
                parsed_blocks.push(result);
                cur_id += 1;
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    let solver = Solver::new(parsed_blocks, board);

    solver.solve();

    let result = solver.get_solution_states();

    if result.is_empty() {
        println!("There is no solution for this puzzle");
        return;
    }

    println!("----------------");
    for (i, solution) in solver.get_solution_states().iter().enumerate() {
        println!("Solution {}", i + 1);
        for r in solution.get_board().get_contents() {
            println!("{}", String::from_iter(r));
        }
        println!("----------------");
    }

    #[cfg(target_os = "windows")]
    Command::new("cmd.exe").arg("/c").arg("pause").status().ok();
}
