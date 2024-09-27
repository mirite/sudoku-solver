#[cfg(test)]
mod tests {
    use crate::input::{read_grid, Cell};
    use crate::solving::solve_grid;
    use std::fs::read_to_string;

    #[test]
    fn can_solve() {
        let data = read_to_string("test_grids/easyToSolve.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        compare_grids(result.unwrap(), "test_grids/easyToSolveSolved.txt")
    }

    #[test]
    fn can_solve_2() {
        let data = read_to_string("test_grids/hardToSolve.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        compare_grids(result.unwrap(), "test_grids/hardToSolveSolved.txt");
    }

    #[test]
    fn can_solve_empty_grid() {
        // With speculative solving, it should actually be possible to solve a grid of all empties.
        let data = read_to_string("test_grids/allEmpty.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_none(), false);
    }

    fn compare_grids(result: [[Cell; 9]; 9], actual_path: &str) {
        let solved_data = read_to_string(actual_path).unwrap();
        let solved_grid = read_grid(solved_data).unwrap();
        for r in 0..9 {
            for c in 0..9 {
                print!("{}", result[r][c].provided);
                if c % 3 == 2 && c != 8 {
                    print!("|");
                    assert_eq!(result[r][c].provided, solved_grid[r][c].provided);
                }
            }
            print!("\n");
            if r % 3 == 2 && r != 8 {
                println!("-----------")
            }
        }
    }
}
