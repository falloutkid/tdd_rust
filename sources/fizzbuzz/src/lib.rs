pub fn fizz_buzz(num: i32) -> String {
    let mut result = match num {
        num if num % 3 == 0 && num % 5 == 0 => "FizzBuzz".to_string(),
        num if num % 3 == 0 => "Fizz".to_string(),
        num if num % 5 == 0 => "Buzz".to_string(),
        _ => "".to_string(),
    };
    let num_str = num.to_string().chars().collect::<Vec<char>>();

    if num_str.contains(&'3') {
        result.push_str("Fizz");
    }

    if num_str.contains(&'5') {
        result.push_str("Buzz");
    }

    if result.is_empty() {
        result.push_str(&num.to_string());
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz_divisible_by_3() {
        assert_eq!(fizz_buzz(6), String::from("Fizz"));
        assert_eq!(fizz_buzz(9), String::from("Fizz"));
        assert_eq!(fizz_buzz(12), String::from("Fizz"));
    }

    #[test]
    fn test_fizz_buzz_divisible_by_5() {
        assert_eq!(fizz_buzz(10), String::from("Buzz"));
        assert_eq!(fizz_buzz(20), String::from("Buzz"));
    }

    #[test]
    fn test_fizz_buzz_divisible_by_3_and_5() {
        assert_eq!(fizz_buzz(60), String::from("FizzBuzz"));
        assert_eq!(fizz_buzz(90), String::from("FizzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_not_divisible_by_3_or_5() {
        assert_eq!(fizz_buzz(1), String::from("1"));
        assert_eq!(fizz_buzz(11), String::from("11"));
    }

    #[test]
    fn test_fizz_buzz_contains_3_only() {
        assert_eq!(fizz_buzz(13), String::from("Fizz"));
        assert_eq!(fizz_buzz(23), String::from("Fizz"));
    }

    #[test]
    fn test_fizz_buzz_contains_3_and_divisible_by_3_and_5() {
        assert_eq!(fizz_buzz(30), String::from("FizzBuzzFizz"));
    }

    #[test]
    fn test_fizz_buzz_contains_3_and_divisible_by_3() {
        assert_eq!(fizz_buzz(3), String::from("FizzFizz"));
    }

    #[test]
    fn test_fizz_buzz_contains_5_only() {
        assert_eq!(fizz_buzz(52), String::from("Buzz"));
    }

    #[test]
    fn test_fizz_buzz_contains_5_and_divisible_by_3_and_5() {
        assert_eq!(fizz_buzz(15), String::from("FizzBuzzBuzz"));
        assert_eq!(fizz_buzz(150), String::from("FizzBuzzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_contains_5_and_divisible_by_3() {
        assert_eq!(fizz_buzz(51), String::from("FizzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_contains_5_and_divisible_by_5() {
        assert_eq!(fizz_buzz(5), String::from("BuzzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_contains_3_and_5_only() {
        assert_eq!(fizz_buzz(53), String::from("FizzBuzz"));
        assert_eq!(fizz_buzz(503), String::from("FizzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_35_special_case() {
        assert_eq!(fizz_buzz(35), String::from("BuzzFizzBuzz"));
    }
}
