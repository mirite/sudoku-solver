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

        test_grid[0][4].provided = 8;
        test_grid[4][2].provided = 8;
        test_grid[0][5].provided = 9;
        test_grid[2][5].provided = 3;
        test_grid[3][1].provided = 3;
        test_grid[4][1].provided = 9;


        test_grid = calculate_possible(test_grid);
        for r in 0..3 {
            for c in 0..3 {
                for n in 1..10 {
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
                        test_grid[r][c].possible[n - 1],
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
        test_grid.print();
    }
}
