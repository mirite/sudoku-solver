use crate::validity::is_valid_grid;

#[derive(Debug, PartialEq)]
pub enum InputError {
    InvalidLineCount,
    InvalidLineLength,
    InvalidLineContent,
    InvalidLayout,
}

pub fn read_grid(content: String) -> Result<[[Cell; 9]; 9], InputError> {
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();

    let mut result_grid: [[Cell; 9]; 9] = [[Cell {
        provided: 0,
        possible: [true; 9],
    }; 9]; 9];

    if line_count < 9 {
        return Err(InputError::InvalidLineCount);
    }

    for r in 0..9 {
        for c in 0..9 {
            let cell_value: Result<usize, InputError> = match lines[r].chars().nth(c) {
                Some(char) => match char.to_digit(10) {
                    Some(digit) => match usize::try_from(digit) {
                        Ok(valid_digit) => Ok(valid_digit),
                        Err(_e) => Err(InputError::InvalidLineContent),
                    },
                    None => Err(InputError::InvalidLineContent),
                },
                None => Err(InputError::InvalidLineLength),
            };
            match cell_value {
                Ok(v) => {
                    result_grid[r][c].provided = v;
                    for p in 0..9 {
                        result_grid[r][c].possible[p] = match v {
                            0 => true,
                            _ => false,
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
    if is_valid_grid(result_grid) {
        Ok(result_grid)
    } else {
        Err(InputError::InvalidLayout)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub provided: usize,
    pub possible: [bool; 9],
}

pub fn print_grid(grid: [[Cell; 9]; 9]) {
    println!("{}", grid_to_string(grid));
}

pub fn grid_to_string(grid: [[Cell; 9]; 9]) -> String {
    let mut result: String = String::from("");
    for r in 0..9 {
        for c in 0..9 {
            result.push_str(&format!("{}", grid[r][c].provided));
            if c % 3 == 2 && c != 8 {
                result.push_str("|");
            }
            if c == 8 {
            result.push_str("\n");
            if r % 3 == 2 && r != 8 {
                result.push_str("-----------\n");
            }
            }
        }
    }

    result
}
