pub fn print_account_number(account_number: &str) -> String {
    let mut account_number = account_number.to_string();
    if account_number.contains('?') {
        account_number.push_str(" ILL");
        account_number
    } else if crate::validator::is_valid_account_number(&account_number) == false {
        account_number.push_str(" ERR");
        account_number
    } else {
        account_number
    }
}

#[cfg(test)]
mod tests_print_result {
    use super::*;
    use crate::recognizer;
    use crate::validator;

    #[test]
    fn test_print_account_number_true() {
        assert_eq!("457508000", print_account_number("457508000"));
    }

    #[test]
    fn test_print_account_number_invalid_checksum() {
        assert_eq!("664371495 ERR", print_account_number("664371495"));
    }

    #[test]
    fn test_print_account_number_invalid_number() {
        assert_eq!("86110??36 ILL", print_account_number("86110??36"));
    }
}
