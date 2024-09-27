use crate::input::Cell;

pub fn is_unsolvable(grid: [[Cell; 9]; 9]) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c].provided != 0 {
                continue;
            }
            let mut has_possible = false;
            for n in 1..9 {
                if grid[r][c].possible[n - 1] {
                    has_possible = true;
                }
            }
            if has_possible == false {
                return true;
            }
        }
    }
    //Still need to check for no possible placements
    false
}
