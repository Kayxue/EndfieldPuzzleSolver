use std::sync::LazyLock;

use regex::Regex;

use crate::components::error::{InvalidBoardError, InvalidNumbersError};

static SPLIT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("Regex create failed"));

pub struct Board {
    row_nums: Vec<u8>,
    column_nums: Vec<u8>,
    state: Vec<Vec<char>>,
}

impl Board {
    pub fn parse_board(rows: Vec<String>) -> Result<Vec<Vec<char>>, InvalidBoardError> {
        if rows.is_empty() {
            return Err(InvalidBoardError::new());
        }

        if rows
            .iter()
            .find(|e| e.chars().any(|c| c != '.' && c != '0' && c != '*') || e.is_empty())
            .is_some()
        {
            return Err(InvalidBoardError::new());
        }

        if let (_, true) = rows.iter().fold((0u8, false), |acc, e| {
            let (cur_length, _) = acc;
            if cur_length == 0 {
                return (e.len() as u8, false);
            }
            if cur_length != e.len() as u8 {
                return (cur_length, true);
            }
            acc
        }) {
            return Err(InvalidBoardError::new());
        }

        let result = rows.iter().map(|e| e.chars().collect()).collect();

        Ok(result)
    }

    pub fn parse_row_nums(
        row_nums_string: String,
        rows: Vec<String>,
    ) -> Result<Vec<u8>, InvalidNumbersError> {
        let row_nums_parse_result: Result<Vec<u8>, _> = SPLIT_REGEX
            .split(&row_nums_string)
            .map(|e| e.parse())
            .collect();

        if row_nums_parse_result.is_err() {
            return Err(InvalidNumbersError::new());
        }

        let actual_row_nums = row_nums_parse_result.unwrap();

        if actual_row_nums.len() != rows.len() {
            return Err(InvalidNumbersError::new());
        }

        Ok(actual_row_nums)
    }

    pub fn parse_column_nums(
        column_nums_string: String,
        rows: Vec<String>,
    ) -> Result<Vec<u8>, InvalidNumbersError> {
        let column_nums_parse_result: Result<Vec<u8>, _> = SPLIT_REGEX
            .split(&column_nums_string)
            .map(|e| e.parse())
            .collect();

        if column_nums_parse_result.is_err() {
            return Err(InvalidNumbersError::new());
        }

        let actual_column_nums = column_nums_parse_result.unwrap();

        if actual_column_nums.len() != rows[0].len() {
            return Err(InvalidNumbersError::new());
        }

        Ok(actual_column_nums)
    }

    pub fn new(row_nums: Vec<u8>, column_nums: Vec<u8>, state: Vec<Vec<char>>) -> Board {
        Board {
            row_nums,
            column_nums,
            state,
        }
    }

    pub fn place_block(&self, block: &Vec<Vec<char>>, (r_fill, c_fill): (usize, usize)) -> Option<Board> {
        let mut placed_board = self.state.clone();
        let mut new_row_nums = self.row_nums.clone();
        let mut new_column_nums = self.column_nums.clone();
        let row_length = placed_board.len();
        let column_length = placed_board[0].len();

        //Find new state
        for (r_index, row) in block.iter().enumerate() {
            for (c_index, c) in row.iter().enumerate() {
                let to_fill_row = r_fill + r_index;
                let to_fill_column = c_fill + c_index;

                //If index out of bound
                if to_fill_row >= row_length || to_fill_column >= column_length {
                    return None;
                }

                //If any pixel has been occupied
                if placed_board[to_fill_row][to_fill_column] != '.' && *c != '.' {
                    return None;
                }

                //If courrent pixel of the block to fill can be fill
                if *c != '.' && new_row_nums[to_fill_row] > 0 && new_column_nums[to_fill_column] > 0
                {
                    placed_board[to_fill_row][to_fill_column] = *c;

                    //Change row_nums and column_nums
                    new_row_nums[to_fill_row] -= 1;
                    new_column_nums[to_fill_column] -= 1;
                } else {
                    return None;
                }
            }
        }

        Some(Board {
            row_nums: new_row_nums,
            column_nums: new_column_nums,
            state: placed_board,
        })
    }
}
