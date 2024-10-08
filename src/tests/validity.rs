#[cfg(test)]
mod tests {
    use crate::input::{read_grid, Cell, InputError};
    use crate::validity::{is_valid_cell_value, is_valid_grid};
    use crate::BLANK_CELL_VALUE;
    use std::fs::read_to_string;

    #[test]
    fn is_invalid_for_row() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 0, 3, 1), false)
    }

    #[test]
    fn is_valid_for_row() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 0, 3, 2), true)
    }

    #[test]
    fn is_valid_same_cell() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 0, 0, 1), true)
    }

    #[test]
    fn is_invalid_for_column() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 3, 0, 1), false)
    }

    #[test]
    fn is_valid_for_column() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 3, 0, 2), true)
    }

    #[test]
    fn is_invalid_for_square() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 2, 1, 1), false)
    }

    #[test]
    fn is_invalid_for_square_2() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[3][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 4, 1, 1), false)
    }

    #[test]
    fn is_invalid_for_square_3() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[7][7].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 8, 8, 1), false)
    }

    #[test]
    fn is_valid_for_square() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            value: BLANK_CELL_VALUE,
            candidates: [true; 9],
        }; 9]; 9];
        test_grid[0][0].value = 1;
        assert_eq!(is_valid_cell_value(test_grid, 2, 1, 2), true)
    }

    #[test]
    fn is_valid_full() {
        let data = read_to_string("test_grids/fullGrid.txt").unwrap();
        let result = read_grid(data).unwrap();
        assert_eq!(is_valid_grid(result), true)
    }

    #[test]
    fn is_invalid_full() {
        let data = read_to_string("test_grids/fullInvalidGrid.txt").unwrap();
        let result = read_grid(data);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err(), Some(InputError::InvalidLayout));
    }
}
