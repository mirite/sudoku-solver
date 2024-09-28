use crate::input::Cell;
use crate::{BLANK_CELL_VALUE, GRID_SIZE, GRID_SIZE_RANGE};

pub fn is_solved(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> bool {
    get_unsolved_count(grid) == 0
}

pub fn get_unsolved_count(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> u8 {
    let mut count = 0;
    for r in GRID_SIZE_RANGE {
        for c in GRID_SIZE_RANGE {
            if grid[r][c].value == BLANK_CELL_VALUE {
                count = count + 1;
            }
        }
    }
    count
}
