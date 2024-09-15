use crate::input::Cell;
use std::usize;

pub fn is_valid_grid(grid: [[Cell; 9]; 9]) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if !is_valid(grid, r, c, grid[r][c].provided) {
                return false;
            }
        }
    }
    true
}
pub fn is_valid(grid: [[Cell; 9]; 9], row: usize, column: usize, value: u8) -> bool {
    is_valid_for_row(grid, row, column, value)
        && is_valid_for_column(grid, row, column, value)
        && is_valid_for_square(grid, row, column, value)
}

pub fn is_valid_for_row(grid: [[Cell; 9]; 9], row: usize, column: usize, value: u8) -> bool {
    for col in 0..9 {
        if grid[row][col].provided == value && col != column {
            return false;
        }
    }
    true
}

pub fn is_valid_for_column(grid: [[Cell; 9]; 9], row: usize, column: usize, value: u8) -> bool {
    for r in 0..9 {
        if grid[r][column].provided == value && r != row {
            return false;
        }
    }
    true
}

pub fn is_valid_for_square(grid: [[Cell; 9]; 9], row: usize, column: usize, value: u8) -> bool {
    let r_start = usize::from(row / 3) * 3;
    let r_end = r_start + 3;
    let c_start = usize::from(column / 3) * 3;
    let c_end = c_start + 3;
    for r in r_start..r_end {
        for c in c_start..c_end {
            if grid[r][c].provided == value && r != row && c != column {
                return false;
            }
        }
    }
    true
}
