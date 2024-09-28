#[cfg(test)]
mod tests {
    use crate::input::{read_grid, InputError};
    use crate::{BLANK_CELL_VALUE, GRID_SIZE_RANGE};
    use std::fs::read_to_string;

    #[test]
    fn read_empty_grid() {
        let data = read_to_string("test_grids/allEmpty.txt").unwrap();
        let result = read_grid(data).unwrap();
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                assert_eq!(result[r][c].provided, BLANK_CELL_VALUE);
                for p in GRID_SIZE_RANGE {
                    assert_eq!(result[r][c].possible[p], true);
                }
            }
        }
    }

    #[test]
    fn read_full_grid() {
        let data = read_to_string("test_grids/fullGrid.txt").unwrap();
        let result = read_grid(data).unwrap();
        for r in GRID_SIZE_RANGE {
            for c in GRID_SIZE_RANGE {
                assert_ne!(result[r][c].provided, BLANK_CELL_VALUE);
                for p in GRID_SIZE_RANGE {
                    assert_eq!(result[r][c].possible[p], false);
                }
            }
        }
    }

    #[test]
    fn read_invalid_chars() {
        let data = read_to_string("test_grids/invalidChars.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result, Some(InputError::InvalidLineContent))
    }

    #[test]
    fn read_invalid_line_count() {
        let data = read_to_string("test_grids/invalidLineCount.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result, Some(InputError::InvalidLineCount))
    }

    #[test]
    fn read_invalid_line_length() {
        let data = read_to_string("test_grids/invalidLineLength.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result, Some(InputError::InvalidLineLength))
    }
}
