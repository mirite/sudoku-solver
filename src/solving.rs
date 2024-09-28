use crate::input::Cell;
use crate::possible::{
    calculate_possible_for_cells, get_possible_placements_for_value, is_only_possible_placement,
};
use crate::solved_detection::is_solved;
use crate::unsolvable_detection::{get_possible_count, is_unsolvable};

pub fn solve_grid(mut grid: [[Cell; 9]; 9]) -> Option<[[Cell; 9]; 9]> {
    let mut last_unsolved = 0;
    let mut unsolved = get_unsolved_count(grid);
    while unsolved != last_unsolved && unsolved != 0 {
        last_unsolved = unsolved;
        grid = calculate_possible_for_cells(grid);
        // If there are any cells with no possible value OR if two cells is the same group share
        // one possible value, return None.
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c].provided == 0 {
                    for n in 1..10 {
                        let possible_placements = get_possible_placements_for_value(grid, n);
                        if is_only_possible_placement(possible_placements, r, c) {
                            grid[r][c].provided = n;
                        }
                    }
                }
            }
        }
        unsolved = get_unsolved_count(grid);
    }
    if is_solved(grid) {
        Some(grid)
    } else if is_unsolvable(grid) {
        None
    } else {
        speculative_solve(grid)
    }
}

pub fn speculative_solve(grid: [[Cell; 9]; 9]) -> Option<[[Cell; 9]; 9]> {
    let (unsolved_row, unsolved_column) = get_next_unsolved(grid);

    for value in 1..10 {
        // If value isn't possible for this cell, it isn't a viable future.
        if grid[unsolved_row][unsolved_column].possible[value - 1] == false {
            continue;
        }
        let mut possible_future = grid.clone();
        possible_future[unsolved_row][unsolved_column].provided = value;

        let result = solve_grid(possible_future);
        if result.is_some() {
            return result;
        }
    }
    None
}

pub fn get_unsolved_count(grid: [[Cell; 9]; 9]) -> u8 {
    let mut count = 0;
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].provided == 0 {
                count = count + 1;
            }
        }
    }
    count
}

/// Gets the address of the first cell with the lowest number of possible values that doesn't have a known value.
fn get_next_unsolved(grid: [[Cell; 9]; 9]) -> (usize, usize) {
    let mut next_row = 0;
    let mut next_col = 0;
    let mut possibles = 10;
    for row in 0..9 {
        for column in 0..9 {
            if grid[row][column].provided == 0 {
                let (possible_value_count_for_cell, _) = get_possible_count(grid, row, column);
                if possibles > possible_value_count_for_cell {
                    next_col = column;
                    next_row = row;
                    possibles = possible_value_count_for_cell
                }
            }
        }
    }
    (next_row, next_col)
}
