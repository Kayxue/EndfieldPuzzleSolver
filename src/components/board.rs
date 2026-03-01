use std::sync::LazyLock;

use regex::Regex;

use crate::{
    components::error::{InvalidBoardError, InvalidNumbersError},
    types::{BlockContent, BoardContent, RequirementNums, RequirementNumsAllColors, StringInputs},
};

static SPLIT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("Regex create failed"));

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Board {
    row_nums: RequirementNumsAllColors,
    column_nums: RequirementNumsAllColors,
    content: BoardContent,
}

impl Board {
    pub fn parse_board(rows: &StringInputs) -> Result<BoardContent, InvalidBoardError> {
        if rows.is_empty() {
            return Err(InvalidBoardError::new());
        }

        // Check input has invalid character
        if rows
            .iter()
            .find(|e| e.chars().any(|c| c != '.' && !c.is_digit(10) && c != '*') || e.is_empty())
            .is_some()
        {
            return Err(InvalidBoardError::new());
        }

        // Check whether input is a matrix
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
        color: &u8,
        row_nums_string: String,
        rows: &BoardContent,
    ) -> Result<RequirementNums, InvalidNumbersError> {
        let row_nums_parse_result: Result<RequirementNums, _> = SPLIT_REGEX
            .split(&row_nums_string)
            .map(|e| e.parse())
            .collect();

        if row_nums_parse_result.is_err() {
            return Err(InvalidNumbersError::new());
        }

        let actual_row_nums = row_nums_parse_result.unwrap();

        if actual_row_nums.iter().all(|e| *e == 0) {
            return Err(InvalidNumbersError::new());
        }

        if actual_row_nums.len() != rows.len() {
            return Err(InvalidNumbersError::new());
        }

        if actual_row_nums.iter().enumerate().any(|(index, row_num)| {
            rows[index]
                .iter()
                .filter(|e| **e == '.' || (**e != '*' && (**e as u8 - '0' as u8) == *color))
                .count()
                < (*row_num as usize)
        }) {
            return Err(InvalidNumbersError::new());
        }

        Ok(actual_row_nums)
    }

    pub fn parse_column_nums(
        color: &u8,
        column_nums_string: String,
        rows: &BoardContent,
    ) -> Result<RequirementNums, InvalidNumbersError> {
        let column_nums_parse_result: Result<RequirementNums, _> = SPLIT_REGEX
            .split(&column_nums_string)
            .map(|e| e.parse())
            .collect();

        if column_nums_parse_result.is_err() {
            return Err(InvalidNumbersError::new());
        }

        let actual_column_nums = column_nums_parse_result.unwrap();

        if actual_column_nums.iter().all(|e| *e == 0) {
            return Err(InvalidNumbersError::new());
        }

        if actual_column_nums.len() != rows[0].len() {
            return Err(InvalidNumbersError::new());
        }

        if actual_column_nums
            .iter()
            .enumerate()
            .any(|(c_index, c_num)| {
                rows.iter()
                    .map(|e| e[c_index])
                    .filter(|e| *e == '.' || (*e != '*' && (*e as u8 - '0' as u8) == *color))
                    .count()
                    < (*c_num as usize)
            })
        {
            return Err(InvalidNumbersError::new());
        }

        Ok(actual_column_nums)
    }

    pub fn new(
        row_nums: RequirementNumsAllColors,
        column_nums: RequirementNumsAllColors,
        initial_board: &BoardContent,
    ) -> Result<Board, InvalidNumbersError> {
        let mut actual_row_numbers = row_nums;
        let mut actual_column_numbers = column_nums;

        // Get actual rol/col requirement and check color exist
        for (r_index, row) in initial_board.iter().enumerate() {
            for (c_index, c) in row.iter().enumerate() {
                if c.is_digit(10) {
                    let color = (*c as u8 - '0' as u8) as usize;
                    if color >= actual_column_numbers.len() {
                        return Err(InvalidNumbersError::new());
                    }
                    actual_row_numbers[color][r_index] -= 1;
                    actual_column_numbers[color][c_index] -= 1;
                }
            }
        }

        // Check column numbers valid
        let column_count = actual_column_numbers[0].len();
        let sum_each_column: Vec<u8> = (0..column_count)
            .map(|i| actual_column_numbers.iter().map(|e| e[i]).sum::<u8>())
            .collect();
        if sum_each_column
            .iter()
            .enumerate()
            .any(|(i, col_num_total)| {
                initial_board
                    .iter()
                    .map(|r| r[i])
                    .filter(|c| *c == '.')
                    .count()
                    < (*col_num_total) as usize
            })
        {
            return Err(InvalidNumbersError::new());
        }

        //Check row numbers valid
        let row_count = actual_row_numbers[0].len();
        let sum_each_row: Vec<u8> = (0..row_count)
            .map(|i| actual_row_numbers.iter().map(|e| e[i]).sum::<u8>())
            .collect();
        if sum_each_row
            .iter()
            .zip(initial_board)
            .any(|(row_num_total, row)| {
                row.iter().filter(|c| **c == '.').count() < (*row_num_total) as usize
            })
        {
            return Err(InvalidNumbersError::new());
        }

        Ok(Board {
            row_nums: actual_row_numbers,
            column_nums: actual_column_numbers,
            content: initial_board.clone(),
        })
    }

    pub fn place_block(
        &self,
        id: &char,
        color: &u8,
        block: &BlockContent,
        (r_fill, c_fill): (usize, usize),
    ) -> Option<Board> {
        let mut new_row_nums = self.row_nums.clone();
        let mut new_column_nums = self.column_nums.clone();

        // Check whether new block can be place, also calculate new row/col nums
        for (r_index, row) in block.iter().enumerate() {
            for (c_index, c) in row.iter().enumerate() {
                if *c == '.' {
                    continue;
                }

                let to_fill_row = r_fill + r_index;
                let to_fill_column = c_fill + c_index;

                //If any pixel has been occupied
                if self.content[to_fill_row][to_fill_column] != '.' {
                    return None;
                }

                //If specific row/column have no pixels can be fill
                if new_row_nums[*color as usize][to_fill_row] <= 0
                    || new_column_nums[*color as usize][to_fill_column] <= 0
                {
                    return None;
                }

                //Change row_nums and column_nums
                new_row_nums[*color as usize][to_fill_row] -= 1;
                new_column_nums[*color as usize][to_fill_column] -= 1;
            }
        }

        let mut placed_board = self.content.clone();

        // Place the block
        for (r_index, row) in block.iter().enumerate() {
            for (c_index, c) in row.iter().enumerate() {
                if *c != '.' {
                    placed_board[r_fill + r_index][c_fill + c_index] = *id;
                }
            }
        }

        Some(Board {
            row_nums: new_row_nums,
            column_nums: new_column_nums,
            content: placed_board,
        })
    }

    pub fn get_colors_count(&self) -> u8 {
        self.column_nums.len() as u8
    }

    pub fn get_contents(&self) -> &BoardContent {
        &self.content
    }

    pub fn get_width(&self) -> u8 {
        self.column_nums[0].len() as u8
    }

    pub fn get_height(&self) -> u8 {
        self.row_nums[0].len() as u8
    }

    pub fn is_row_all_zero(&self) -> bool {
        self.row_nums
            .iter()
            .all(|color| color.iter().all(|e| *e == 0))
    }

    pub fn is_column_all_zero(&self) -> bool {
        self.column_nums
            .iter()
            .all(|color| color.iter().all(|e| *e == 0))
    }
}
