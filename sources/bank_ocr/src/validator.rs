pub fn is_valid_account_number(account_number: &str) -> bool {
    let mut check_sum = 0;
    let base = '0' as i32;
    for (i, v) in account_number.bytes().rev().enumerate() {
        check_sum += (v as i32 - base) * (i as i32 + 1);
    }
    check_sum % 11 == 0
}

#[cfg(test)]
mod tests_check_sum {
    use super::*;

    #[test]
    fn test_check_sum_true() {
        assert!(is_valid_account_number("123456789"));
        assert!(is_valid_account_number("345882865"));
    }

    #[test]
    fn test_check_sum_false() {
        assert!(!is_valid_account_number("123456780"));
    }
}