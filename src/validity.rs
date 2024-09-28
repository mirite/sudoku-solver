use crate::input::Cell;
use crate::math_helpers::get_square_ranges;
use crate::{BLANK_CELL_VALUE, GRID_SIZE, GRID_SIZE_RANGE};
use std::usize;

pub fn is_valid_grid(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> bool {
    for row in GRID_SIZE_RANGE {
        for column in GRID_SIZE_RANGE {
            if grid[row][column].value != BLANK_CELL_VALUE
                && !is_valid_cell_value(grid, row, column, grid[row][column].value)
            {
                return false;
            }
        }
    }
    true
}

pub fn is_valid_cell_value(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
    value: usize,
) -> bool {
    is_valid_for_row(grid, cell_row, cell_column, value)
        && is_valid_for_column(grid, cell_row, cell_column, value)
        && is_valid_for_square(grid, cell_row, cell_column, value)
}

pub fn is_valid_for_row(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
    value: usize,
) -> bool {
    for same_row_column in GRID_SIZE_RANGE {
        if grid[cell_row][same_row_column].value == value && same_row_column != cell_column {
            return false;
        }
    }
    true
}

pub fn is_valid_for_column(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
    value: usize,
) -> bool {
    for same_column_row in GRID_SIZE_RANGE {
        if grid[same_column_row][cell_column].value == value && same_column_row != cell_row {
            return false;
        }
    }
    true
}

pub fn is_valid_for_square(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    cell_row: usize,
    cell_column: usize,
    value: usize,
) -> bool {
    let (r_range, c_range) = get_square_ranges(cell_row, cell_column);
    for same_square_row in r_range {
        for same_square_column in c_range.clone() {
            if grid[same_square_row][same_square_column].value == value
                && same_square_row != cell_row
                && same_square_column != cell_column
            {
                return false;
            }
        }
    }
    true
}
