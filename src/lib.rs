mod input;
mod math_helpers;
mod possible;
mod solved_detection;
mod solving;
mod tests;
mod unsolvable_detection;
mod validity;

use crate::input::grid_to_string;
use crate::input::read_grid;
use crate::solving::solve_grid;
use wasm_bindgen::prelude::*;

const GRID_SIZE_RANGE: std::ops::Range<usize> = 0..9;
const CELL_VALUE_RANGE: std::ops::Range<usize> = 1..10;
const BLANK_CELL_VALUE: usize = 0;

#[wasm_bindgen]
pub fn solve_puzzle(text: String) -> JsValue {
    let grid = read_grid(text);
    let result = match grid {
        Ok(g) => solve_grid(g),
        Err(_e) => return JsValue::from_str("The provided grid was invalid"),
    };

    match result {
        Some(j) => JsValue::from_str(&grid_to_string(j)),
        None => JsValue::from_str("No solution found"),
    }
}
