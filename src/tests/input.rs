#[cfg(test)]
mod tests {
    use crate::input::{read_grid, InputError};
    use std::fs::read_to_string;

    #[test]
    fn read_empty_grid() {
        let data = read_to_string("test_grids/allEmpty.txt").unwrap();
        let result = read_grid(data).unwrap();
        for r in 0..9 {
            for c in 0..9 {
                assert_eq!(result[r][c].provided,0);
                for p in 0..9 {
                    assert_eq!(result[r][c].possible[p],true);
                }
            }
        }
    }

    #[test]
    fn read_full_grid() {
        let data = read_to_string("test_grids/fullGrid.txt").unwrap();
        let result = read_grid(data).unwrap();
        for r in 0..9 {
            for c in 0..9 {
                assert_ne!(result[r][c].provided,0);
                for p in 0..9 {
                    assert_eq!(result[r][c].possible[p],false);
                }
            }
        }
    }

    #[test]
    fn read_invalid_chars() {
        let data = read_to_string("test_grids/invalidChars.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result,Some(InputError::InvalidLineContent))

    }

    #[test]
    fn read_invalid_line_count() {
        let data = read_to_string("test_grids/invalidLineCount.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result,Some(InputError::InvalidLineCount))

    }

    #[test]
    fn read_invalid_line_length() {
        let data = read_to_string("test_grids/invalidLineLength.txt").unwrap();
        let result = read_grid(data).err();
        assert_eq!(result,Some(InputError::InvalidLineLength))

    }
}
