use std::result;

const ZERO_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', ' ', '|'], ['|', '_', '|']];
const ONE_PATTERN: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', '|'], [' ', ' ', '|']];
const TWO_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', '_', '|'], ['|', '_', ' ']];
const THREE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', '_', '|'], [' ', '_', '|']];
const FOUR_PATTERN: [[char; 3]; 3] = [[' ', ' ', ' '], ['|', '_', '|'], [' ', ' ', '|']];
const FIVE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', ' '], [' ', '_', '|']];
const SIX_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', ' '], ['|', '_', '|']];
const SEVEN_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', ' ', '|'], [' ', ' ', '|']];
const EIGHT_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', '|'], ['|', '_', '|']];
const NINE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', '|'], [' ', '_', '|']];

fn recognize_digit(pattern: [[char; 3]; 3]) -> char {
    match pattern {
        ZERO_PATTERN => '0',
        // 他の数字のパターンもここに追加していくイメージ
        _ => '?', // 認識できない場合は '?' を返すとか
    }
}

fn cat_number(line: &str, index: usize) -> [[char; 3]; 3] {
    let mut result: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let lines = line.split("\n");
    let mut lines_char: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut line_char: Vec<char> = Vec::new();
        for c in line.chars() {
            line_char.push(c);
        }
        lines_char.push(line_char);
    }

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = lines_char[i][index * 3 + j];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_number() {
        let numbers =
            "    _  _     _  _  _  _  _ \n  | _| _||_||_ |_   ||_||_|\n  ||_  _|  | _||_|  ||_| _|";
        let one = cat_number(&numbers, 0);
        assert_eq!(one, ONE_PATTERN);
        let two = cat_number(&numbers, 1);
        assert_eq!(two, TWO_PATTERN);
        let three = cat_number(&numbers, 2);
        assert_eq!(three, THREE_PATTERN);
        let four = cat_number(&numbers, 3);
        assert_eq!(four, FOUR_PATTERN);
        let six = cat_number(&numbers, 5);
        assert_eq!(six, SIX_PATTERN);
        let seven = cat_number(&numbers, 6);
        assert_eq!(seven, SEVEN_PATTERN);
        let eight = cat_number(&numbers, 7);
        assert_eq!(eight, EIGHT_PATTERN);
        let nine = cat_number(&numbers, 8);
        assert_eq!(nine, NINE_PATTERN);
    }

    #[test]
    fn test_recognize_digit_zero() {
        assert_eq!(recognize_digit(ZERO_PATTERN), '0');
    }
}
