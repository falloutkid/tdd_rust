pub const ZERO_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', ' ', '|'], ['|', '_', '|']];
pub const ONE_PATTERN: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', '|'], [' ', ' ', '|']];
pub const TWO_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', '_', '|'], ['|', '_', ' ']];
pub const THREE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', '_', '|'], [' ', '_', '|']];
pub const FOUR_PATTERN: [[char; 3]; 3] = [[' ', ' ', ' '], ['|', '_', '|'], [' ', ' ', '|']];
pub const FIVE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', ' '], [' ', '_', '|']];
pub const SIX_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', ' '], ['|', '_', '|']];
pub const SEVEN_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], [' ', ' ', '|'], [' ', ' ', '|']];
pub const EIGHT_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', '|'], ['|', '_', '|']];
pub const NINE_PATTERN: [[char; 3]; 3] = [[' ', '_', ' '], ['|', '_', '|'], [' ', '_', '|']];

pub fn recognize_digit(pattern: [[char; 3]; 3]) -> char {
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
        _ => '?',
    }
}

pub fn recognize_account_number(numbers: &str) -> String {
    let mut result = String::new();
    for i in 0..9 {
        let pattern = cat_number(numbers, i);
        result.push(recognize_digit(pattern));
    }
    result
}

pub fn cat_number(line: &str, index: usize) -> [[char; 3]; 3] {
    let mut result: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let lines_char: Vec<Vec<char>> = line
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = lines_char[i][index * 3 + j];
        }
    }

    result
}

pub fn generate_one_off_patterns(pattern: [[char; 3]; 3]) -> Vec<char> {
    fn diff_count(a: &[[char; 3]; 3], b: &[[char; 3]; 3]) -> usize {
        let mut count = 0;
        for c in 0..3 {
            for r in 0..3 {
                if a[c][r] != b[c][r] {
                    count += 1;
                }
            }
        }
        count
    }
    let numbers = [
        ZERO_PATTERN,
        ONE_PATTERN,
        TWO_PATTERN,
        THREE_PATTERN,
        FOUR_PATTERN,
        FIVE_PATTERN,
        SIX_PATTERN,
        SEVEN_PATTERN,
        EIGHT_PATTERN,
        NINE_PATTERN,
    ];
    let mut result = Vec::new();

    for (index, number) in numbers.iter().enumerate() {
        if diff_count(&pattern, &number) <= 1 {
            result.push((index as u8 + b'0' as u8) as char);
        }
    }
    result
}

#[cfg(test)]
mod tests_common {
    use crate::recognizer::*;

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
    fn test_recognize_all_digits() {
        let test_cases = [
            (ZERO_PATTERN, '0'),
            (ONE_PATTERN, '1'),
            (TWO_PATTERN, '2'),
            (THREE_PATTERN, '3'),
            (FOUR_PATTERN, '4'),
            (FIVE_PATTERN, '5'),
            (SIX_PATTERN, '6'),
            (SEVEN_PATTERN, '7'),
            (EIGHT_PATTERN, '8'),
            (NINE_PATTERN, '9'),
        ];

        for (pattern, expected) in test_cases.iter() {
            assert_eq!(recognize_digit(*pattern), *expected);
        }
    }
}

#[cfg(test)]
mod tests_recognize_account_number {
    use super::*;

    #[test]
    fn test_parse_account_number_zero() {
        let zero_account_number_pattern = " _  _  _  _  _  _  _  _  _ \n| || || || || || || || || |\n|_||_||_||_||_||_||_||_||_|\n                           "; // 最後の行は空白だよ

        assert_eq!(
            "000000000",
            recognize_account_number(&zero_account_number_pattern)
        );
    }

    #[test]
    fn test_parse_account_number_11111111() {
        let zero_account_number_pattern = "                           \n  |  |  |  |  |  |  |  |  |\n  |  |  |  |  |  |  |  |  |\n                           "; // 最後の行は空白だよ

        assert_eq!(
            "111111111",
            recognize_account_number(&zero_account_number_pattern)
        );
    }

    #[test]
    fn test_parse_account_number_123456789() {
        let zero_account_number_pattern = "    _  _     _  _  _  _  _ \n  | _| _||_||_ |_   ||_||_|\n  ||_  _|  | _||_|  ||_| _|\n                           "; // 最後の行は空白だよ

        assert_eq!(
            "123456789",
            recognize_account_number(&zero_account_number_pattern)
        );
    }
}

#[cfg(test)]
mod tests_one_off_patterns {
    use super::*;

    #[test]
    fn test_generate_one_off_patterns() {
        // Test Case 1: '0' -> '8' (add char)
        let zero_pattern = ZERO_PATTERN;
        let one_off_patterns_from_zero = generate_one_off_patterns(zero_pattern);
        assert!(one_off_patterns_from_zero.contains(&('8' as char)));

        // Test Case 2: '8' -> '0' (remove char)
        let eight_pattern = EIGHT_PATTERN;
        let one_off_patterns_from_eight = generate_one_off_patterns(eight_pattern);
        assert!(one_off_patterns_from_eight.contains(&('0' as char)));

        // Test Case 3: '1' -> '7' (add char)
        let one_pattern = ONE_PATTERN;
        let one_off_patterns_from_one = generate_one_off_patterns(one_pattern);
        assert!(one_off_patterns_from_one.contains(&('7' as char)));

        // Test Case 4: '7' -> '1' (remove char)
        let seven_pattern = SEVEN_PATTERN;
        let one_off_patterns_from_seven = generate_one_off_patterns(seven_pattern);
        assert!(one_off_patterns_from_seven.contains(&('1' as char)));

        // Test Case 5: '5' -> '6' and '9' (multiple candidates)
        let five_pattern = FIVE_PATTERN;
        let one_off_patterns_from_five = generate_one_off_patterns(five_pattern);
        assert!(one_off_patterns_from_five.contains(&('6' as char)));
        assert!(one_off_patterns_from_five.contains(&('9' as char)));
    }
}
