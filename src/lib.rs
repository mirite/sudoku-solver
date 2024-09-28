mod input;
mod math_helpers;
mod possible;
mod solved_detection;
mod solving;
mod tests;
mod unsolvable_detection;
mod validity;

use crate::input::read_grid;
use crate::solving::solve_grid;
use wasm_bindgen::prelude::*;

const GRID_SIZE: usize = 9;
const GRID_SIZE_RANGE: std::ops::Range<usize> = 0..GRID_SIZE;
const CELL_VALUE_RANGE: std::ops::Range<usize> = 1..GRID_SIZE + 1;
const BLANK_CELL_VALUE: usize = 0;

#[wasm_bindgen]
pub fn solve_puzzle(text: String) -> JsValue {
    let grid = read_grid(text);
    let result = match grid {
        Ok(grid_from_input) => solve_grid(grid_from_input),
        Err(_e) => return JsValue::from_str("The provided grid was invalid"),
    };

    match result {
        Some(solved_grid) => {
            let mut result: String = String::from("");
            for row in GRID_SIZE_RANGE {
                for column in GRID_SIZE_RANGE {
                    result.push_str(&format!("{}", solved_grid[row][column].value))
                }
            }
            JsValue::from_str(&result)
        }
        None => JsValue::from_str("No solution found"),
    }
}
