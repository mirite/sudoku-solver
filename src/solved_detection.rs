use crate::input::Cell;
use crate::solving::get_unsolved_count;
use crate::GRID_SIZE;

pub fn is_solved(grid: [[Cell; GRID_SIZE]; GRID_SIZE]) -> bool {
    get_unsolved_count(grid) == 0
}
