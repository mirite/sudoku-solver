#[cfg(test)]
mod tests {
    use crate::input::Cell;
    use crate::unsolvable_detection::is_unsolvable;

    #[test]
    fn has_two_possibles() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 0,
            possible: [false; 9],
        }; 9]; 9];

        //Two cells in the same row where the only possible value is one.
        test_grid[0][0].possible[0] = true;
        test_grid[3][0].possible[0] = true;
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_no_possibles() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 0,
            possible: [true; 9],
        }; 9]; 9];

        //A cell with no possible values.
        for n in 1..10 {
            test_grid[0][0].possible[n - 1] = false;
        }
        assert_eq!(is_unsolvable(test_grid), true);
    }

    #[test]
    fn has_valid_possibles() {
        let test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 0,
            possible: [true; 9],
        }; 9]; 9];

        assert_eq!(is_unsolvable(test_grid), false);
    }
}
