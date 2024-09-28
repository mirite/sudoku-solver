use crate::input::Cell;
use crate::math_helpers::get_square_ranges;
use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE, GRID_SIZE_RANGE};

/// Determines if a grid is in an unsolvable state.
pub fn is_unsolvable(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> bool {
    for grid_row in GRID_SIZE_RANGE {
        for grid_column in GRID_SIZE_RANGE {
            if grid[grid_row][grid_column].value != BLANK_CELL_VALUE {
                continue;
            }
            let (possible_values, only_possible_value) =
                get_possible_count(grid, grid_row, grid_column);
            if possible_values == 0 {
                return true;
            }

            if only_possible_value != 0 {
                // Check if there are any other cells where there is only the one possible value.
                //Check for other cells in the same row
                for same_row_column in GRID_SIZE_RANGE {
                    if is_conflicting_cell(grid, grid_row, grid_column, grid_row, same_row_column) {
                        return true;
                    }
                }

                //Check for other cells in the same column
                for same_column_row in GRID_SIZE_RANGE {
                    if is_conflicting_cell(
                        grid,
                        grid_row,
                        grid_column,
                        same_column_row,
                        grid_column,
                    ) {
                        return true;
                    }
                }

                //Check for other cells in the same square
                let (r_range, c_range) = get_square_ranges(grid_row, grid_column);
                for same_square_row in r_range {
                    for same_square_column in c_range.clone() {
                        if is_conflicting_cell(
                            grid,
                            grid_row,
                            grid_column,
                            same_square_row,
                            same_square_column,
                        ) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

/// Determines if two cells both require their value to be the same number.
fn is_conflicting_cell(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    current_row: usize,
    current_column: usize,
    checking_against_row: usize,
    checking_against_column: usize,
) -> bool {
    let (_, last_possible_value) = get_possible_count(grid, current_row, current_column);
    if checking_against_row == current_row && checking_against_column == current_column {
        return false;
    }
    let (possible_values_row, last_possible_value_row) =
        get_possible_count(grid, checking_against_row, checking_against_column);
    possible_values_row == 1 && last_possible_value_row == last_possible_value
}

/// Gets the number of possible values for the cell. If that number is equal to one,
/// the second item in the tuple will be that only possible value.
pub fn get_possible_count(
    grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    row: usize,
    col: usize,
) -> (usize, usize) {
    let mut possible_value_count: usize = 0;
    let mut last_possible_value: usize = 0;
    for value in CELL_VALUE_RANGE {
        if grid[row][col].candidates[value - 1] {
            possible_value_count = possible_value_count + 1;
            last_possible_value = value;
        }
    }
    (
        possible_value_count,
        if possible_value_count == 1 {
            last_possible_value
        } else {
            0
        },
    )
}
