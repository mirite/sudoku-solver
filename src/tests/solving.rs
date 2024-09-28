#[cfg(test)]
mod tests {
    use crate::input::{read_grid, Cell};
    use crate::solving::solve_grid;
    use crate::{GRID_SIZE, GRID_SIZE_RANGE};
    use std::fs::read_to_string;

    #[test]
    fn can_solve() {
        let data = read_to_string("test_grids/easyToSolve.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_some(), true);
        compare_grids(result.unwrap(), "test_grids/easyToSolveSolved.txt")
    }

    #[test]
    fn can_solve_2() {
        let data = read_to_string("test_grids/hardToSolve.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_some(), true);
        compare_grids(result.unwrap(), "test_grids/hardToSolveSolved.txt");
    }

    #[test]
    fn can_solve_3() {
        let data = read_to_string("test_grids/extremeToSolve.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_some(), true);
        compare_grids(result.unwrap(), "test_grids/extremeToSolveSolved.txt");
    }

    #[test]
    fn can_solve_4() {
        let data = read_to_string("test_grids/extremeToSolve2.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_some(), true);
        compare_grids(result.unwrap(), "test_grids/extremeToSolve2Solved.txt");
    }

    #[test]
    fn can_solve_empty_grid() {
        // With speculative solving, it should actually be possible to solve a grid of all empties.
        let data = read_to_string("test_grids/allEmpty.txt").unwrap();
        let grid = read_grid(data).unwrap();
        let result = solve_grid(grid);
        assert_eq!(result.is_none(), false);
    }

    fn compare_grids(result: [[Cell; GRID_SIZE]; GRID_SIZE], actual_path: &str) {
        let solved_data = read_to_string(actual_path).unwrap();
        let solved_grid = read_grid(solved_data).unwrap();
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                assert_eq!(
                    result[r][c].value, solved_grid[r][c].value,
                    "Expected the value at ({},{}) to be {}. Got {}",
                    r, c, solved_grid[r][c].value, result[r][c].value
                );
            }
        }
    }
}
