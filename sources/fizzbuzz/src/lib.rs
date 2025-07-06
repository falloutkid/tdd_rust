pub fn fizz_buzz(num: i32) -> String {
    match num {
        num if num % 15 == 0 => "FizzBuzz".to_string(),
        num if num % 3 == 0 => "Fizz".to_string(),
        num if num % 5 == 0 => "Buzz".to_string(),
        _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz_divisible_by_3() {
        assert_eq!(fizz_buzz(3), String::from("Fizz"));
        assert_eq!(fizz_buzz(6), String::from("Fizz"));
        assert_eq!(fizz_buzz(9), String::from("Fizz"));
        assert_eq!(fizz_buzz(12), String::from("Fizz"));
    }

    #[test]
    fn test_fizz_buzz_divisible_by_5() {
        assert_eq!(fizz_buzz(5), String::from("Buzz"));
        assert_eq!(fizz_buzz(10), String::from("Buzz"));
        assert_eq!(fizz_buzz(20), String::from("Buzz"));
    }

    #[test]
    fn test_fizz_buzz_divisible_by_3_and_5() {
        assert_eq!(fizz_buzz(15), String::from("FizzBuzz"));
        assert_eq!(fizz_buzz(30), String::from("FizzBuzz"));
        assert_eq!(fizz_buzz(45), String::from("FizzBuzz"));
    }

    #[test]
    fn test_fizz_buzz_not_divisible_by_3_or_5() {
        assert_eq!(fizz_buzz(1), String::from("1"));
        assert_eq!(fizz_buzz(11), String::from("11"));
        assert_eq!(fizz_buzz(13), String::from("13"));
    }
}