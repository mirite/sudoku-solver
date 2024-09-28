use crate::input::Cell;
use crate::math_helpers::get_square_ranges;
use crate::validity::is_valid_cell_value;
use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE, GRID_SIZE_RANGE};

pub fn calculate_candidates_for_cells(
    mut grid: [[Cell; GRID_SIZE]; GRID_SIZE],
) -> [[Cell; GRID_SIZE]; GRID_SIZE] {
    for row in GRID_SIZE_RANGE {
        for column in GRID_SIZE_RANGE {
            for value in CELL_VALUE_RANGE {
                grid[row][column].candidates[value - 1] = grid[row][column].value
                    == BLANK_CELL_VALUE
                    && is_valid_cell_value(grid, row, column, value);
            }
        }
    }
    grid
}

pub fn get_possible_placements_for_value(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    value: usize,
) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut result = [[false; GRID_SIZE]; GRID_SIZE];
    for row in GRID_SIZE_RANGE {
        for column in GRID_SIZE_RANGE {
            result[row][column] = grid[row][column].candidates[value - 1];
        }
    }
    result
}

fn is_only_possible_in_row(grid: [[bool; GRID_SIZE]; GRID_SIZE], row: usize) -> bool {
    let mut is_first_possible = true;
    for column in GRID_SIZE_RANGE {
        if grid[row][column] {
            if !is_first_possible {
                return false;
            }
            is_first_possible = false;
        }
    }
    !is_first_possible
}

fn is_only_possible_in_column(grid: [[bool; GRID_SIZE]; GRID_SIZE], column: usize) -> bool {
    let mut is_first_possible = true;
    for row in GRID_SIZE_RANGE {
        if grid[row][column] {
            if !is_first_possible {
                return false;
            }
            is_first_possible = false;
        }
    }
    !is_first_possible
}

fn is_only_possible_in_square(
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
) -> bool {
    let (r_range, c_range) = get_square_ranges(cell_row, cell_column);
    let mut is_first_possible = true;
    for checking_row in r_range {
        for checking_column in c_range.clone() {
            if grid[checking_row][checking_column] {
                if !is_first_possible {
                    return false;
                }
                is_first_possible = false;
            }
        }
    }

    !is_first_possible
}

pub fn is_only_possible_placement(
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
) -> bool {
    if !grid[cell_row][cell_column] {
        return false;
    }
    let only_in_row = is_only_possible_in_row(grid, cell_row);
    let only_in_column = is_only_possible_in_column(grid, cell_column);
    let only_in_square = is_only_possible_in_square(grid, cell_row, cell_column);
    only_in_row || only_in_column || only_in_square
}
