use crate::input::Cell;
use crate::validity::is_valid;

pub fn calculate_possible_for_cells(mut grid: [[Cell; 9]; 9]) -> [[Cell; 9]; 9] {
    for r in 0..9 {
        for c in 0..9 {
            for n in 1..10 {
                grid[r][c].possible[n - 1] = grid[r][c].provided == 0 && is_valid(grid, r, c, n);
            }
        }
    }
    grid
}

pub fn get_possible_placements_for_value(grid: [[Cell; 9]; 9], value: usize) -> [[bool; 9]; 9] {
    let mut result = [[false; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            result[r][c] = grid[r][c].possible[value - 1];
        }
    }
    result
}

fn is_only_possible_in_row(grid: [[bool; 9]; 9], row: usize) -> bool {
    let mut first = true;
    for col in 0..9 {
        if grid[row][col] {
            if !first {
                return false;
            }
            first = false;
        }
    }
    !first
}

fn is_only_possible_in_column(grid: [[bool; 9]; 9], column: usize) -> bool {
    let mut first = true;
    for row in 0..9 {
        if grid[row][column] {
            if !first {
                return false;
            }
            first = false;
        }
    }
    !first
}

fn is_only_possible_in_square(grid: [[bool; 9]; 9], row: usize, column: usize) -> bool {
    let r_start = usize::from(row / 3) * 3;
    let r_end = r_start + 3;
    let c_start = usize::from(column / 3) * 3;
    let c_end = c_start + 3;
    let mut first = true;
    for r in r_start..r_end {
        for c in c_start..c_end {
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

pub fn is_only_possible_placement(grid: [[bool; 9]; 9], row: usize, column: usize) -> bool {
    if !grid[row][column] {
        return false;
    }
    let only_in_row = is_only_possible_in_row(grid, row);
    let only_in_column = is_only_possible_in_column(grid, column);
    let only_in_square = is_only_possible_in_square(grid, row, column);
    only_in_row || only_in_column || only_in_square
}
