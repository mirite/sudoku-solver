#[cfg(test)]
mod tests {
    use crate::input::Cell;
    use crate::unsolvable_detection::is_unsolvable;
    use crate::{BLANK_CELL_VALUE, CELL_VALUE_RANGE};

    #[test]
    fn has_two_possibles_in_column() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: 1,
            candidates: [false; 9],
        }; 9]; 9];

        //Two cells in the same column where the only possible value is one.
        test_grid[0][0].candidates[0] = true;
        test_grid[0][0].value = BLANK_CELL_VALUE;
        test_grid[3][0].candidates[0] = true;
        test_grid[3][0].value = BLANK_CELL_VALUE;
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_two_possibles_in_row() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: 1,
            candidates: [false; 9],
        }; 9]; 9];

        //Two cells in the same column where the only possible value is one.
        test_grid[0][0].candidates[0] = true;
        test_grid[0][0].value = BLANK_CELL_VALUE;
        test_grid[0][3].candidates[0] = true;
        test_grid[0][3].value = BLANK_CELL_VALUE;
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_two_possibles_in_square() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: 1,
            candidates: [false; 9],
        }; 9]; 9];

        //Two cells in the same column where the only possible value is one.
        test_grid[0][0].candidates[0] = true;
        test_grid[0][0].value = BLANK_CELL_VALUE;
        test_grid[2][2].candidates[0] = true;
        test_grid[2][2].value = BLANK_CELL_VALUE;
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_no_possibles() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];

        //A cell with no possible values.
        for n in CELL_VALUE_RANGE {
            test_grid[0][0].candidates[n - 1] = false;
        }
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_valid_possibles() {
        let test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];

        assert_eq!(is_unsolvable(test_grid), false);
    }
}
