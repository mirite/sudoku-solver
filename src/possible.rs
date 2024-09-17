use crate::input::Cell;
use crate::validity::is_valid;

pub fn calculate_possible(mut grid: [[Cell; 9]; 9]) -> [[Cell; 9]; 9] {
    for r in 0..9 {
        for c in 0..9 {
            for n in 1..10 {
                grid[r][c].possible[n - 1] = grid[r][c].provided == 0 && is_valid(grid, r, c, n);
            }
        }
    }
    grid
}

pub fn fill_inferred(mut grid: [[Cell; 9]; 9]) -> [[Cell; 9]; 9] {
    let mut last_unsolved = 0;
    let mut unsolved = get_unsolved_count(grid);
    while unsolved != last_unsolved {
        last_unsolved = unsolved;
        grid = calculate_possible(grid);
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c].provided != 0 {
                    let mut value_count = 0;
                    let mut last_available = 0;
                    for n in 1..10 {
                        if grid[r][c].possible[n - 1] == true {
                            value_count = value_count + 1;
                            last_available = n - 1;
                        }
                    }
                    //Or if it's the only possible place for that value
                    if value_count == 1 {
                        grid[r][c].provided = last_available;
                    }
                }
            }
        }
        unsolved = get_unsolved_count(grid);
    }
    grid
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
