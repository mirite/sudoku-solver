#[cfg(test)]
mod tests {
    use crate::input::{read_grid, Cell};
    use crate::possible::{
        calculate_candidates_for_cells, get_possible_placements_for_value,
        is_only_possible_placement,
    };

    use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE, GRID_SIZE_RANGE};
    use std::fs::read_to_string;

    #[test]
    fn can_calculate_possible_values() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];

        test_grid[0][0].value = 1;
        test_grid[0][1].value = 2;
        // Checking for row
        test_grid[1][0].value = 4;
        test_grid[1][1].value = 5;
        test_grid[1][2].value = 6;
        test_grid[2][0].value = 7;
        // Checking for column
        //Checking for cell

        test_grid[0][4].value = 8;
        test_grid[4][2].value = 8;
        test_grid[0][5].value = 9;
        test_grid[2][5].value = 3;
        test_grid[3][1].value = 3;
        test_grid[4][1].value = 9;

        test_grid = calculate_candidates_for_cells(test_grid);
        for r in 0..3 {
            for c in 0..3 {
                for n in CELL_VALUE_RANGE {
                    let expected = if r == 0 && c == 2 && n == 3 {
                        true
                    } else if r == 2 && c == 1 && n == 8 {
                        true
                    } else if r == 2 && c == 2 && n == 9 {
                        true
                    } else {
                        false
                    };
                    assert_eq!(
                        test_grid[r][c].candidates[n - 1],
                        expected,
                        "Expected for {} to be {} at ({},{})",
                        n,
                        expected,
                        r,
                        c
                    );
                }
            }
        }
    }

    #[test]
    fn calculate_possible_placements() {
        let data = read_to_string("test_grids/easyToSolve.txt").unwrap();
        let mut grid = read_grid(data).unwrap();
        grid = calculate_candidates_for_cells(grid);
        let possibles = get_possible_placements_for_value(grid, 7);
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                let expected = if (r == 2 && c == 2)
                    || (r == 5 && c == 3)
                    || (r == 7 && c == 0)
                    || (r == 7 && c == 2)
                    || (r == 7 && c == 7)
                    || (r == 8 && c == 2)
                    || (r == 8 && c == 7)
                {
                    true
                } else {
                    false
                };
                assert_eq!(
                    possibles[r][c], expected,
                    "Expected for {} to be {} at ({},{})",
                    possibles[r][c], expected, r, c
                )
            }
        }
    }

    #[test]
    fn can_handle_every_possible() {
        let data = read_to_string("test_grids/allEmpty.txt").unwrap();
        let mut grid = read_grid(data).unwrap();
        grid = calculate_candidates_for_cells(grid);
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                for n in CELL_VALUE_RANGE {
                    assert_eq!(grid[r][c].candidates[n - 1], true);
                }
            }
        }
    }

    #[test]
    fn test_is_only_possible_placement() {
        let data = read_to_string("test_grids/easyToSolve.txt").unwrap();
        let mut grid = read_grid(data).unwrap();
        grid = calculate_candidates_for_cells(grid);
        let possibles = get_possible_placements_for_value(grid, 7);
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                let expected = if (r == 2 && c == 2) || (r == 5 && c == 3) || (r == 7 && c == 0) {
                    true
                } else {
                    false
                };
                let result = is_only_possible_placement(possibles, r, c);
                assert_eq!(
                    result, expected,
                    "Expected for {} to be {} at ({},{})",
                    result, expected, r, c
                );
            }
        }
    }
}
