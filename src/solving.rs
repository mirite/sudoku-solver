use crate::input::Cell;
use crate::possible::{
    calculate_candidates_for_cells, get_possible_placements_for_value, is_only_possible_placement,
};
use crate::solved_detection::is_solved;
use crate::unsolvable_detection::{get_possible_count, is_unsolvable};
use crate::{solved_detection, BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE, GRID_SIZE_RANGE};

pub fn solve_grid(
    mut grid: [[Cell; GRID_SIZE]; GRID_SIZE],
) -> Option<[[Cell; GRID_SIZE]; GRID_SIZE]> {
    let mut last_unsolved = 0;
    let mut unsolved = solved_detection::get_unsolved_count(grid);
    while unsolved != last_unsolved && unsolved != 0 {
        last_unsolved = unsolved;
        for row in GRID_SIZE_RANGE {
            for column in GRID_SIZE_RANGE {
                grid = calculate_candidates_for_cells(grid);
                if grid[row][column].value == BLANK_CELL_VALUE {
                    for value in CELL_VALUE_RANGE {
                        let possible_placements = get_possible_placements_for_value(grid, value);
                        if is_only_possible_placement(possible_placements, row, column) {
                            grid[row][column].value = value;
                        }
                    }
                }
            }
        }
        unsolved = solved_detection::get_unsolved_count(grid);
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
        if grid[unsolved_row][unsolved_column].candidates[value - 1] == false {
            continue;
        }
        let mut possible_future = grid.clone();
        possible_future[unsolved_row][unsolved_column].value = value;

        let result = solve_grid(possible_future);
        if result.is_some() {
            return result;
        }
    }
    None
}

/// Gets the address of the first cell with the lowest number of possible values that doesn't have a known value.
fn get_next_unsolved(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> (usize, usize) {
    let mut next_cell = (0, 0);
    let mut fewest_possible_options = GRID_SIZE + 1; // Start with one more than the actual maximum number of possibles.
    for row in GRID_SIZE_RANGE {
        for column in GRID_SIZE_RANGE {
            if grid[row][column].value == BLANK_CELL_VALUE {
                let (possible_options_for_cell, _) = get_possible_count(grid, row, column);
                if fewest_possible_options > possible_options_for_cell {
                    next_cell = (row, column);
                    fewest_possible_options = possible_options_for_cell
                }
            }
        }
    }
    next_cell
}
