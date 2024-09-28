use crate::input::Cell;
use crate::possible::{
    calculate_possible_for_cells, get_possible_placements_for_value, is_only_possible_placement,
};
use crate::solved_detection::is_solved;
use crate::unsolvable_detection::{get_possible_count, is_unsolvable};
use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE, GRID_SIZE_RANGE};

pub fn solve_grid(
    mut grid: [[Cell; GRID_SIZE]; GRID_SIZE],
) -> Option<[[Cell; GRID_SIZE]; GRID_SIZE]> {
    let mut last_unsolved = 0;
    let mut unsolved = get_unsolved_count(grid);
    while unsolved != last_unsolved && unsolved != 0 {
        last_unsolved = unsolved;
        // If there are any cells with no possible value OR if two cells is the same group share
        // one possible value, return None.
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                grid = calculate_possible_for_cells(grid); //
                if grid[r][c].provided == BLANK_CELL_VALUE {
                    for n in CELL_VALUE_RANGE {
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

pub fn speculative_solve(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
) -> Option<[[Cell; GRID_SIZE]; GRID_SIZE]> {
    let (unsolved_row, unsolved_column) = get_next_unsolved(grid);

    for value in CELL_VALUE_RANGE {
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

pub fn get_unsolved_count(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> u8 {
    let mut count = 0;
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            if grid[r][c].provided == BLANK_CELL_VALUE {
                count = count + 1;
            }
        }
    }
    count
}

/// Gets the address of the first cell with the lowest number of possible values that doesn't have a known value.
fn get_next_unsolved(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> (usize, usize) {
    let mut next_row = 0;
    let mut next_col = 0;
    let mut possibles = GRID_SIZE + 1; // Start with one more than the actual maximum number of possibles.
    for row in GRID_SIZE_RANGE {
        for column in GRID_SIZE_RANGE {
            if grid[row][column].provided == BLANK_CELL_VALUE {
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
