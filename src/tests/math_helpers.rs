#[cfg(test)]
mod tests {
    use crate::math_helpers::get_square_ranges;

    #[test]
    fn can_get_ranges() {
        assert_eq!(get_square_ranges(0, 0), (0..3, 0..3));
        assert_eq!(get_square_ranges(1, 1), (0..3, 0..3));
        assert_eq!(get_square_ranges(2, 2), (0..3, 0..3));
        assert_eq!(get_square_ranges(3, 3), (3..6, 3..6));
        assert_eq!(get_square_ranges(4, 4), (3..6, 3..6));
        assert_eq!(get_square_ranges(5, 5), (3..6, 3..6));
        assert_eq!(get_square_ranges(6, 6), (6..9, 6..9));
        assert_eq!(get_square_ranges(7, 7), (6..9, 6..9));
        assert_eq!(get_square_ranges(8, 8), (6..9, 6..9));
        assert_eq!(get_square_ranges(8, 0), (6..9, 0..3));
        assert_eq!(get_square_ranges(7, 1), (6..9, 0..3));
        assert_eq!(get_square_ranges(6, 2), (6..9, 0..3));
        assert_eq!(get_square_ranges(5, 3), (3..6, 3..6));
        assert_eq!(get_square_ranges(3, 5), (3..6, 3..6));
        assert_eq!(get_square_ranges(2, 6), (0..3, 6..9));
        assert_eq!(get_square_ranges(1, 7), (0..3, 6..9));
        assert_eq!(get_square_ranges(0, 8), (0..3, 6..9));
    }
}
