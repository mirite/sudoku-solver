use crate::input::Cell;
use crate::math_helpers::get_square_ranges;
use crate::{BLANK_CELL_VALUE, GRID_SIZE, GRID_SIZE_RANGE};
use std::usize;

pub fn is_valid_grid(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> bool {
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            if grid[r][c].provided != BLANK_CELL_VALUE && !is_valid(grid, r, c, grid[r][c].provided)
            {
                return false;
            }
        }
    }
    true
}
pub fn is_valid(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
    value: usize,
) -> bool {
    is_valid_for_row(grid, row, column, value)
        && is_valid_for_column(grid, row, column, value)
        && is_valid_for_square(grid, row, column, value)
}

pub fn is_valid_for_row(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
    value: usize,
) -> bool {
    for col in GRID_SIZE_RANGE {
        if grid[row][col].provided == value && col != column {
            return false;
        }
    }
    true
}

pub fn is_valid_for_column(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
    value: usize,
) -> bool {
    for r in GRID_SIZE_RANGE {
        if grid[r][column].provided == value && r != row {
            return false;
        }
    }
    true
}

pub fn is_valid_for_square(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
    value: usize,
) -> bool {
    let (r_range, c_range) = get_square_ranges(row, column);
    for r in r_range {
        for c in c_range.clone() {
            if grid[r][c].provided == value && r != row && c != column {
                return false;
            }
        }
    }
    true
}
