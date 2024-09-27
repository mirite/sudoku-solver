pub fn get_square_ranges(row: usize, column: usize) -> (usize, usize, usize, usize) {
    let r_start = usize::from(row / 3) * 3;
    let r_end = r_start + 3;
    let c_start = usize::from(column / 3) * 3;
    let c_end = c_start + 3;
    (r_start, r_end, c_start, c_end)
}
