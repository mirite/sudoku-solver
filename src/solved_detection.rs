use crate::input::Cell;
use crate::solving::get_unsolved_count;

pub fn is_solved(grid: [[Cell; 9]; 9]) -> bool {
    get_unsolved_count(grid) == 0
}
