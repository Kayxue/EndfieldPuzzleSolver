use std::{collections::HashSet, iter};

use crate::{
    components::error::InvalidBlockError,
    types::{BlockContent, StringInputs},
};

#[derive(Debug)]
pub struct Block {
    id: char,
    color: u8,
    pixels: HashSet<BlockContent>,
    filled_pixels_count: u8,
}

impl Block {
    pub fn new(id: char, rows: StringInputs) -> Result<Block, InvalidBlockError> {
        // Empty Block
        if rows.is_empty() {
            return Err(InvalidBlockError::new());
        }

        // Check invalid symbols
        if rows
            .iter()
            .find(|e| {
                e.chars().any(|c| c != '.' && !c.is_digit(10))
                    || e.is_empty()
                    || e.chars()
                        .filter(|e| e.is_digit(10))
                        .collect::<HashSet<char>>()
                        .len()
                        == 0
            })
            .is_some()
        {
            return Err(InvalidBlockError::new());
        }

        //Check block pixels contain only one number
        if rows
            .iter()
            .find(|e| {
                e.chars()
                    .filter(|e| e.is_digit(10))
                    .collect::<HashSet<char>>()
                    .len()
                    > 1
            })
            .is_some()
        {
            return Err(InvalidBlockError::new());
        }

        let pixel_digit = rows[0].chars().find(|e| e.is_digit(10)).unwrap();

        let longest_row_length = rows.iter().fold(0, |acc, e| {
            let last_digit_location = e.rfind(pixel_digit).unwrap();
            if last_digit_location > acc {
                return last_digit_location;
            }
            acc
        }) + 1;

        let mut pixels: BlockContent = rows
            .into_iter()
            .map(|s| {
                s.chars()
                    .take(longest_row_length)
                    .chain(iter::repeat('.'))
                    .take(longest_row_length)
                    .collect()
            })
            .collect();

        let filled_pixel_total = pixels
            .iter()
            .map(|v| v.iter().filter(|e| **e == pixel_digit).count())
            .sum::<usize>() as u8;

        let mut all_state: HashSet<BlockContent> = HashSet::new();

        for _ in 0..4 {
            all_state.insert(pixels.clone());
            pixels = Block::get_rotate_result(pixels);
        }

        Ok(Block {
            id: id,
            color: pixel_digit as u8 - '0' as u8,
            pixels: all_state,
            filled_pixels_count: filled_pixel_total,
        })
    }

    fn get_rotate_result(matrix: BlockContent) -> BlockContent {
        let new_row_length = matrix[0].len();

        (0..new_row_length)
            .map(|col_index| {
                // For each column in the original matrix, read the rows from bottom to top
                matrix
                    .iter()
                    .rev() // Read rows in reverse (bottom to top)
                    .map(|row| row[col_index]) // Pluck out the character for this column
                    .collect()
            })
            .collect()
    }

    pub fn get_id(&self) -> &char {
        &self.id
    }

    pub fn get_color(&self) -> &u8 {
        &self.color
    }

    pub fn get_block_rotate_state(&self) -> &HashSet<BlockContent> {
        &self.pixels
    }

    pub fn get_filled_pixels_count(&self) -> u8 {
        self.filled_pixels_count
    }
}
