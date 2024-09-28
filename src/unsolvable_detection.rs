use crate::input::{print_grid, Cell};
use crate::math_helpers::get_square_ranges;

/// Determines if a grid is in an unsolvable state.
pub fn is_unsolvable(grid: [[Cell; 9]; 9]) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].provided != 0 {
                continue;
            }
            let (possible_values, only_possible_value) = get_possible_count(grid, r, c);
            if possible_values == 0 {
                return true;
            }

            if only_possible_value != 0 {
                // Check if there are any other cells where there is only the one possible value.
                //Check for other cells in the same row
                for col in 0..9 {
                    if is_conflicting_cell(grid, r, c, r, col) {
                        return true;
                    }
                }

                //Check for other cells in the same coumn
                for row in 0..9 {
                    if is_conflicting_cell(grid, r, c, row, c) {
                        print_grid(grid);
                        return true;
                    }
                }

                //Check for other cells in the same square
                let (r_start, r_end, c_start, c_end) = get_square_ranges(r, c);
                for row in r_start..r_end {
                    for column in c_start..c_end {
                        if is_conflicting_cell(grid, r, c, row, column) {
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
    grid: [[Cell; 9]; 9],
    current_row: usize,
    current_column: usize,
    checking_against_row: usize,
    checking_against_column: usize,
) -> bool {
    let (_possible_values_row, last_possible_value) =
        get_possible_count(grid, current_row, current_column);
    if checking_against_row == current_row && checking_against_column == current_column {
        return false;
    }
    let (possible_values_row, last_possible_value_row) =
        get_possible_count(grid, checking_against_row, checking_against_column);
    possible_values_row == 1 && last_possible_value_row == last_possible_value
}

/// Gets the number of possible values for the cell. If that number is equal to one,
/// the second item in the tuple will be that only possible value.
pub fn get_possible_count(grid: [[Cell; 9]; 9], row: usize, col: usize) -> (usize, usize) {
    let mut possible_values: usize = 0;
    let mut last_possible_value: usize = 0;
    for n in 1..10 {
        if grid[row][col].possible[n - 1] {
            possible_values = possible_values + 1;
            last_possible_value = n;
        }
    }
    (
        possible_values,
        if possible_values == 1 {
            last_possible_value
        } else {
            0
        },
    )
}
