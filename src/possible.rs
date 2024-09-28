use crate::input::Cell;
use crate::math_helpers::get_square_ranges;
use crate::validity::is_valid;
use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE, GRID_SIZE_RANGE};

pub fn calculate_possible_for_cells(
    mut grid: [[Cell; GRID_SIZE]; GRID_SIZE],
) -> [[Cell; GRID_SIZE]; GRID_SIZE] {
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            for n in CELL_VALUE_RANGE {
                grid[r][c].possible[n - 1] =
                    grid[r][c].provided == BLANK_CELL_VALUE && is_valid(grid, r, c, n);
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
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            result[r][c] = grid[r][c].possible[value - 1];
        }
    }
    result
}

fn is_only_possible_in_row(grid: [[bool; GRID_SIZE]; GRID_SIZE], row: usize) -> bool {
    let mut first = true;
    for col in GRID_SIZE_RANGE {
        if grid[row][col] {
            if !first {
                return false;
            }
            first = false;
        }
    }
    !first
}

fn is_only_possible_in_column(grid: [[bool; GRID_SIZE]; GRID_SIZE], column: usize) -> bool {
    let mut first = true;
    for row in GRID_SIZE_RANGE {
        if grid[row][column] {
            if !first {
                return false;
            }
            first = false;
        }
    }
    !first
}

fn is_only_possible_in_square(
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
) -> bool {
    let (r_range, c_range) = get_square_ranges(row, column);
    let mut first = true;
    for r in r_range {
        for c in c_range.clone() {
            if grid[r][c] {
                if !first {
                    return false;
                }
                first = false;
            }
        }
    }

    !first
}

pub fn is_only_possible_placement(
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    row: usize,
    column: usize,
) -> bool {
    if !grid[row][column] {
        return false;
    }
    let only_in_row = is_only_possible_in_row(grid, row);
    let only_in_column = is_only_possible_in_column(grid, column);
    let only_in_square = is_only_possible_in_square(grid, row, column);
    only_in_row || only_in_column || only_in_square
}
