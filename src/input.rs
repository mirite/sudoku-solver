use crate::unsolvable_detection::is_unsolvable;
use crate::validity::is_valid_grid;
use crate::GRID_SIZE_RANGE;
use crate::{BLANK_CELL_VALUE, GRID_SIZE};

#[derive(Debug, PartialEq)]
pub enum InputError {
    InvalidLineCount,
    InvalidLineLength,
    InvalidLineContent,
    InvalidLayout,
}

pub fn read_grid(content: String) -> Result<[[Cell; GRID_SIZE]; GRID_SIZE], InputError> {
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();

    let mut result_grid: [[Cell; GRID_SIZE]; GRID_SIZE] = [[Cell {
        provided: BLANK_CELL_VALUE,
        possible: [true; GRID_SIZE],
    }; GRID_SIZE]; GRID_SIZE];

    if line_count < GRID_SIZE {
        return Err(InputError::InvalidLineCount);
    }

    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            let cell_value: Result<usize, InputError> = match lines[r].chars().nth(c) {
                Some(char) => match char.to_digit(10) {
                    Some(digit) => match usize::try_from(digit) {
                        Ok(valid_digit) => Ok(valid_digit),
                        Err(_e) => Err(InputError::InvalidLineContent),
                    },
                    None => Err(InputError::InvalidLineContent),
                },
                None => Err(InputError::InvalidLineLength),
            };
            match cell_value {
                Ok(v) => {
                    result_grid[r][c].provided = v;
                    for p in GRID_SIZE_RANGE {
                        result_grid[r][c].possible[p] = match v {
                            BLANK_CELL_VALUE => true,
                            _ => false,
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
    if is_valid_grid(result_grid) && !is_unsolvable(result_grid) {
        Ok(result_grid)
    } else {
        Err(InputError::InvalidLayout)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub provided: usize,
    pub possible: [bool; GRID_SIZE],
}

pub fn print_grid(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) {
    println!("{}", grid_to_string(grid));
}

pub fn grid_to_string(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> String {
    let mut result: String = String::from("");
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            result.push_str(&format!("{}", grid[r][c].provided));
            if c % 3 == 2 && c != 8 {
                result.push_str("|");
            }
            if c == 8 {
                result.push_str("\n");
                if r % 3 == 2 && r != 8 {
                    result.push_str("-----------\n");
                }
            }
        }
    }

    result
}
