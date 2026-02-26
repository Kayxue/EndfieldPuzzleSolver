use std::{collections::HashSet, iter};

use crate::components::error::InvalidBlockError;

#[derive(Debug)]
pub struct Block {
    id: char,
    pixels: HashSet<Vec<Vec<char>>>,
}

impl Block {
    pub fn new(id: char, rows: Vec<String>) -> Result<Block, InvalidBlockError> {
        if rows.is_empty() {
            return Err(InvalidBlockError::new());
        }

        if let Some(_) = rows
            .iter()
            .find(|e| e.chars().any(|c| c != '.' && c != '0') || e.is_empty() || !e.contains('0'))
        {
            return Err(InvalidBlockError::new());
        }

        let longest_row_length = rows.iter().fold(0, |acc, e| {
            if e.rfind('0').unwrap() > acc {
                return e.len();
            }
            acc
        });

        let mut pixels: Vec<Vec<char>> = rows
            .into_iter()
            .map(|s| {
                s.chars()
                    .take(longest_row_length)
                    .chain(iter::repeat('.'))
                    .take(longest_row_length)
                    .collect()
            })
            .collect();

        let mut all_state: HashSet<Vec<Vec<char>>> = HashSet::new();

        for _ in 0..4 {
            all_state.insert(pixels.clone());
            pixels = Block::get_rotate_result(pixels);
        }

        Ok(Block {
            id: id,
            pixels: all_state,
        })
    }

    fn get_rotate_result(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let new_row_length = matrix[0].len();

        (0..new_row_length)
            .map(|col_index| {
                // For each column in the original matrix, read the rows from bottom to top
                matrix
                    .iter()
                    .rev() // Read rows in reverse (bottom to top)
                    .map(|row| row[col_index]) // Pluck out the character for this column
                    .collect::<Vec<char>>()
            })
            .collect()
    }

    pub fn get_block_states(&self) -> &HashSet<Vec<Vec<char>>> {
        &self.pixels
    }

    pub fn get_id(&self) -> &char {
        &self.id
    }
}
