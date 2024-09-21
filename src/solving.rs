use crate::input::Cell;
use crate::possible;

pub fn fill_inferred(mut grid: [[Cell; 9]; 9]) -> Option<[[Cell; 9]; 9]> {
    let mut last_unsolved = 0;
    let mut unsolved = get_unsolved_count(grid);
    while unsolved != last_unsolved {
        last_unsolved = unsolved;
        grid = possible::calculate_possible_for_cells(grid);
        // If there are any cells with no possible value OR if two cells is the same group share
        // one possible value, return None.
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c].provided == 0 {
                    for n in 1..10 {
                        let possible_placements =
                            possible::get_possible_placements_for_value(grid, n);
                        if possible::is_only_possible_placement(possible_placements, r, c) {
                            grid[r][c].provided = n;
                        }
                    }
                }
            }
        }
        unsolved = get_unsolved_count(grid);
    }
    // If the unsolved is 0, return the grid.
    // cycle through the possible values in the first unsolved cell recursivly, one of the possible
    // values WILL be correct.
    Some(grid)
}

fn get_unsolved_count(grid: [[Cell; 9]; 9]) -> i32 {
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
