use crate::input::Cell;

pub fn is_solved(grid: [[Cell; 9]; 9]) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].provided == 0 {
                return false;
            }
        }
    }
    true
}
