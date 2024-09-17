use crate::input::{Cell};
use crate::validity::is_valid;

pub fn calculate_possible(mut grid: [[Cell; 9]; 9]) -> [[Cell; 9]; 9] {
    for r in 0..9 {
        for c in 0..9 {
            for n in 1..10 {
                grid[r][c].possible[n-1] = grid[r][c].provided == 0 && is_valid(grid, r, c, n);
            }
        }
    }
    grid
}
