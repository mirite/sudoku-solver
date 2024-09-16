#[cfg(test)]
mod tests {
    use crate::input::Cell;
    use crate::possible::calculate_possible;

    #[test]
    fn can_calculate_possible_values() {
        let mut test_grid: [[Cell; 9]; 9] = [[Cell {
            provided: 0,
            possible: [true; 9],
        }; 9]; 9];

        test_grid[0][0].provided = 1;
        test_grid[0][1].provided = 2;
        // Checking for row
        test_grid[1][0].provided = 4;
        test_grid[1][1].provided = 5;
        test_grid[1][2].provided = 6;
        test_grid[2][0].provided = 7;
        // Checking for column
        //Checking for cell

        test_grid = calculate_possible(test_grid);
        assert_eq!(test_grid[0][2].possible[3 - 1], true); //Need to check for all the falses too.
        assert_eq!(test_grid[2][1].possible[8 - 1], true);
        assert_eq!(test_grid[2][2].possible[9 - 1], true);
    }
}
