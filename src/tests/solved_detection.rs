#[cfg(test)]
mod tests {
    use crate::input::Cell;
    use crate::solved_detection::is_solved;

    #[test]
    fn can_check_solved() {
        let test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 1,
            possible: [true; 9],
        }; 9]; 9];
        assert_eq!(is_solved(test_grid), true);
    }
    #[test]
    fn can_check_unsolved() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 1,
            possible: [true; 9],
        }; 9]; 9];
        test_grid[8][8].provided = 0;
        assert_eq!(is_solved(test_grid), false);
    }
}
