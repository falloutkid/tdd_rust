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
        ONE_PATTERN => '1',
        TWO_PATTERN => '2',
        THREE_PATTERN => '3',
        FOUR_PATTERN => '4',
        FIVE_PATTERN => '5',
        SIX_PATTERN => '6',
        SEVEN_PATTERN => '7',
        EIGHT_PATTERN => '8',
        NINE_PATTERN => '9',
        _ => '?', // 認識できない場合は '?' を返すとか
    }
}

fn recognize_account_number(numbers: &str) -> String {
    let mut result = String::new();
    for i in 0..9 {
        let pattern = cat_number(numbers, i);
        result.push(recognize_digit(pattern));
    }
    result
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
mod tests_common {
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
}

#[cfg(test)]
mod tests_recognize_number {
    use super::*;

    #[test]
    fn test_recognize_digit_zero() {
        assert_eq!(recognize_digit(ZERO_PATTERN), '0');
    }
    #[test]
    fn test_recognize_digit_one() {
        assert_eq!(recognize_digit(ONE_PATTERN), '1');
    }
    #[test]
    fn test_recognize_digit_two() {
        assert_eq!(recognize_digit(TWO_PATTERN), '2');
    }
    #[test]
    fn test_recognize_digit_three() {
        assert_eq!(recognize_digit(THREE_PATTERN), '3');
    }
    #[test]
    fn test_recognize_digit_four() {
        assert_eq!(recognize_digit(FOUR_PATTERN), '4');
    }
    #[test]
    fn test_recognize_digit_five() {
        assert_eq!(recognize_digit(FIVE_PATTERN), '5');
    }
    #[test]
    fn test_recognize_digit_six() {
        assert_eq!(recognize_digit(SIX_PATTERN), '6');
    }
    #[test]
    fn test_recognize_digit_seven() {
        assert_eq!(recognize_digit(SEVEN_PATTERN), '7');
    }
    #[test]
    fn test_recognize_digit_eight() {
        assert_eq!(recognize_digit(EIGHT_PATTERN), '8');
    }
    #[test]
    fn test_recognize_digit_nine() {
        assert_eq!(recognize_digit(NINE_PATTERN), '9');
    }
}

#[cfg(test)]
mod tests_recognize_account_number {
    use super::*;

    #[test]
    fn test_parse_account_number_zero() {
        let zero_account_number_pattern = " _  _  _  _  _  _  _  _  _ 
| || || || || || || || || |
|_||_||_||_||_||_||_||_||_|
                           "; // 最後の行は空白だよ

        assert_eq!("000000000", recognize_account_number(&zero_account_number_pattern));
    }

        #[test]
    fn test_parse_account_number_11111111() {
        let zero_account_number_pattern = "                           
  |  |  |  |  |  |  |  |  |
  |  |  |  |  |  |  |  |  |
                           "; // 最後の行は空白だよ

        assert_eq!("111111111", recognize_account_number(&zero_account_number_pattern));
    }

        #[test]
    fn test_parse_account_number_123456789() {
        let zero_account_number_pattern = "    _  _     _  _  _  _  _ 
  | _| _||_||_ |_   ||_||_|
  ||_  _|  | _||_|  ||_| _|
                           "; // 最後の行は空白だよ

        assert_eq!("123456789", recognize_account_number(&zero_account_number_pattern));
    }
}
