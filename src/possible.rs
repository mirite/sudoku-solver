use crate::input::{Cell, Grid};
use crate::validity::is_valid;

pub fn calculate_possible(mut grid: Grid) -> Grid {
    for r in 0..9 {
        for c in 0..9 {
            for n in 1..10 {
                grid[r][c].possible[n-1] = grid[r][c].provided == 0 && is_valid(grid, r, c, n);
            }
        }
    }
    grid
}
