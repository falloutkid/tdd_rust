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

// 新しい関数をここに実装するよ
pub fn correct_account_number(account_number: &str) -> Vec<String> {
    // 仮の実装。後でちゃんと実装するね！
    vec![account_number.to_string()]
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

    #[test]
    fn test_correct_account_number_single_candidate() {
        // 123456780 は ERR になるはず。0 を 9 に変えると 123456789 で有効になる。
        let account_number = "123456780";
        let result = correct_account_number(account_number);

        assert!(result.contains(&"123456789".to_string()));
    }
}
